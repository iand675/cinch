// Command smtp_vectors records SMTP client transcripts from the reference SDK's implementation.
//
// Each scenario gives a scripted server: a greeting and one reply per client command, sent as the client
// asks. The reference client runs the scenario's operations against it over loopback; the row records the
// scripted replies, the exact bytes the client wrote and each operation's error text (empty for success).
//
//	go run tooling/smtp_vectors.go > seed/tests/fixtures/smtp-vectors.json
package main

import (
	"bufio"
	"encoding/json"
	"net"
	"net/smtp"
	"os"
	"strings"
	"sync"
)

type row struct {
	Name     string   `json:"name"`
	Replies  []string `json:"replies"`
	Client   string   `json:"client"`
	Results  []string `json:"results"`
	Steps    []string `json:"steps"`
}

// serve answers each client line (and each DATA body) with the next scripted reply; it records everything the
// client writes.
func serve(l net.Listener, replies []string, record *strings.Builder, done *sync.WaitGroup) {
	defer done.Done()
	conn, err := l.Accept()
	if err != nil {
		return
	}
	defer conn.Close()
	r := bufio.NewReader(conn)
	i := 0
	send := func() bool {
		if i >= len(replies) {
			return false
		}
		conn.Write([]byte(replies[i]))
		i++
		return true
	}
	send()
	inData := false
	for {
		line, err := r.ReadString('\n')
		record.WriteString(line)
		if err != nil {
			return
		}
		if inData {
			if line == ".\r\n" {
				inData = false
				send()
			}
			continue
		}
		if strings.HasPrefix(replies[min(i, len(replies)-1)], "354") && strings.HasPrefix(strings.ToUpper(line), "DATA") {
			inData = true
		}
		if !send() {
			return
		}
	}
}

func errText(err error) string {
	if err == nil {
		return ""
	}
	return err.Error()
}

type scenario struct {
	name    string
	replies []string
	steps   []string
}

func run(sc scenario) row {
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		panic(err)
	}
	var record strings.Builder
	var done sync.WaitGroup
	done.Add(1)
	go serve(l, sc.replies, &record, &done)
	results := []string{}
	c, err := smtp.Dial(l.Addr().String())
	results = append(results, errText(err))
	if err == nil {
		for _, step := range sc.steps {
			parts := strings.SplitN(step, " ", 2)
			arg := ""
			if len(parts) > 1 {
				arg = parts[1]
			}
			var e error
			switch parts[0] {
			case "hello":
				e = c.Hello(arg)
			case "mail":
				e = c.Mail(arg)
			case "rcpt":
				e = c.Rcpt(arg)
			case "data":
				w, err := c.Data()
				if err == nil {
					_, err = w.Write([]byte(strings.NewReplacer(`\r`, "\r", `\n`, "\n").Replace(arg)))
					if err == nil {
						err = w.Close()
					}
				}
				e = err
			case "plain":
				f := strings.Fields(arg)
				e = c.Auth(smtp.PlainAuth("", f[0], f[1], f[2]))
			case "cram":
				f := strings.Fields(arg)
				e = c.Auth(smtp.CRAMMD5Auth(f[0], f[1]))
			case "verify":
				e = c.Verify(arg)
			case "reset":
				e = c.Reset()
			case "noop":
				e = c.Noop()
			case "extension":
				ok, param := c.Extension(arg)
				if ok {
					e = errorString("ext " + param)
				} else {
					e = errorString("no ext")
				}
			case "quit":
				e = c.Quit()
			}
			results = append(results, errText(e))
		}
		c.Close()
	}
	l.Close()
	done.Wait()
	return row{Name: sc.name, Replies: sc.replies, Client: record.String(), Results: results, Steps: sc.steps}
}

type errorString string

func (e errorString) Error() string { return string(e) }

func main() {
	ehlo := "250-mail.example.org greets you\r\n250-8BITMIME\r\n250-SIZE 35882577\r\n250-AUTH PLAIN CRAM-MD5\r\n250 HELP\r\n"
	scenarios := []scenario{
		{"send", []string{"220 mail.example.org ESMTP ready\r\n", ehlo, "250 2.1.0 Ok\r\n", "250 2.1.5 Ok\r\n", "251 forwarding\r\n", "354 End data with <CR><LF>.<CR><LF>\r\n", "250 2.0.0 queued as 42\r\n", "221 2.0.0 Bye\r\n"},
			[]string{"hello client.example.net", "mail alice@example.org", "rcpt bob@example.net", "rcpt carol@example.net", `data Subject: hi\r\n\r\n.leading dot\nbare newline\r\n..two dots\r\nend`, "quit"}},
		{"helo-fallback", []string{"220 old.example ready\r\n", "502 command not implemented\r\n", "250 old.example\r\n", "250 ok\r\n", "250 ok\r\n", "354 go\r\n", "250 ok\r\n", "221 bye\r\n"},
			[]string{"mail a@b", "rcpt c@d", `data no trailing newline`, "quit"}},
		{"plain-auth", []string{"220 localhost ESMTP\r\n", ehlo, "235 2.7.0 Authentication successful\r\n", "250 ok\r\n", "221 bye\r\n"},
			[]string{"plain user secret 127.0.0.1", "mail me@localhost", "quit"}},
		{"plain-wrong-host", []string{"220 mail.example.org ESMTP\r\n", ehlo, "221 bye\r\n"},
			[]string{"plain user secret mail.example.org"}},
		{"cram-md5", []string{"220 localhost ESMTP\r\n", ehlo, "334 PDQxOTI5NDIzNDEuMTI4Mjg0NzJAc291cmNlZm91ci5hbmRyZXcuY211LmVkdT4=\r\n", "235 ok\r\n", "221 bye\r\n"},
			[]string{"cram tim tanstaaftanstaaf", "quit"}},
		{"auth-failure", []string{"220 localhost ESMTP\r\n", ehlo, "535 5.7.8 Authentication credentials invalid\r\n", "501 aborted\r\n", "221 bye\r\n"},
			[]string{"plain user wrong 127.0.0.1"}},
		{"cram-unexpected-success", []string{"220 localhost ESMTP\r\n", "250-localhost\r\n250 AUTH CRAM-MD5\r\n", "235 already\r\n", "250 ok\r\n", "221 bye\r\n"},
			[]string{"cram tim secret", "noop", "quit"}},
		{"plain-challenged", []string{"220 localhost ESMTP\r\n", ehlo, "334 bW9yZQ==\r\n", "501 aborted\r\n", "221 bye\r\n"},
			[]string{"plain u p 127.0.0.1"}},
		{"hello-after-mail", []string{"220 mx\r\n", "250 mx\r\n", "250 ok\r\n", "221 bye\r\n"},
			[]string{"noop", "hello late", "quit"}},
		{"expect-mismatch-multiline", []string{"220 mx\r\n", "250 mx\r\n", "451-temporary\r\n452 different code\r\n451 last\r\n", "221 bye\r\n"},
			[]string{"noop", "quit"}},
		{"rcpt-rejected", []string{"220 mx ESMTP\r\n", ehlo, "250 ok\r\n", "550-5.1.1 The email account that you tried to reach\r\n550 5.1.1 does not exist\r\n", "250 reset\r\n", "221 bye\r\n"},
			[]string{"mail a@b", "rcpt nobody@b", "reset", "quit"}},
		{"verify-and-extensions", []string{"220 mx ESMTP\r\n", ehlo, "252 cannot verify\r\n", "550 no such user\r\n", "250 ok\r\n", "221 bye\r\n"},
			[]string{"extension size", "extension auth", "extension pipelining", "verify alice", "verify nobody", "noop", "quit"}},
		{"bad-lines", []string{"220 mx ESMTP\r\n", ehlo, "250 ok\r\n", "221 bye\r\n"},
			[]string{"mail a\r\nRCPT TO:<x>", "rcpt b\nc", "verify x\ry", "noop", "quit"}},
		{"greeting-rejected", []string{"554 no service\r\n"}, []string{}},
		{"malformed-reply", []string{"220 mx ESMTP\r\n", "25\r\n"}, []string{"hello x", "noop"}},
		{"data-rejected", []string{"220 mx ESMTP\r\n", "250 mx\r\n", "250 ok\r\n", "250 ok\r\n", "354 go\r\n", "552 too big\r\n", "221 bye\r\n"},
			[]string{"mail a@b", "rcpt c@d", `data line one\r\n.\r\n`, "quit"}},
		{"multiline-mixed", []string{"220-mx first\r\n220-second line\r\n220 third\r\n", "250-mx\r\n250 SMTPUTF8\r\n", "250 ok\r\n", "221 bye\r\n"},
			[]string{"mail utf@example", "quit"}},
		{"empty-data", []string{"220 mx\r\n", "250 mx\r\n", "250 ok\r\n", "250 ok\r\n", "354 go\r\n", "250 ok\r\n", "221 bye\r\n"},
			[]string{"mail a@b", "rcpt c@d", "data ", "quit"}},
	}
	var rows []row
	for _, sc := range scenarios {
		rows = append(rows, run(sc))
	}
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", " ")
	if err := enc.Encode(rows); err != nil {
		panic(err)
	}
}
