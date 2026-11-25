package main

import (
	"crypto"
	"crypto/aes"
	"crypto/cipher"
	"crypto/ecdsa"
	"crypto/ed25519"
	"crypto/elliptic"
	"crypto/hkdf"
	"crypto/hmac"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/sha512"
	"crypto/tls"
	"crypto/x509"
	"crypto/x509/pkix"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"encoding/pem"
	"flag"
	"fmt"
	"hash"
	"io"
	"math/big"
	"net"
	"os"
	"strings"
	"time"
)

// deterministicReader produces deterministic pseudo-random bytes using SHA-256 in counter mode
// so that generated fixture certificates and keys are reproducible across runs.
type deterministicReader struct {
	seed    []byte
	counter uint64
	buf     []byte
}

func newDeterministicReader(label string) *deterministicReader {
	h := sha256.Sum256([]byte(label))
	return &deterministicReader{seed: h[:]}
}

func (r *deterministicReader) Read(p []byte) (int, error) {
	written := 0
	for written < len(p) {
		if len(r.buf) == 0 {
			var ctr [8]byte
			binary.BigEndian.PutUint64(ctr[:], r.counter)
			r.counter++
			h := sha256.New()
			h.Write(r.seed)
			h.Write(ctr[:])
			r.buf = h.Sum(nil)
		}
		n := copy(p[written:], r.buf)
		r.buf = r.buf[n:]
		written += n
	}
	return written, nil
}

func hkdfExtract(h func() hash.Hash, salt, ikm []byte) []byte {
	prk, err := hkdf.Extract(h, ikm, salt)
	if err != nil {
		panic(err)
	}
	return prk
}

func hkdfExpandLabel(h func() hash.Hash, secret []byte, label string, context []byte, length int) []byte {
	fullLabel := "tls13 " + label
	info := make([]byte, 0, 2+1+len(fullLabel)+1+len(context))
	info = append(info, byte(length>>8), byte(length))
	info = append(info, byte(len(fullLabel)))
	info = append(info, fullLabel...)
	info = append(info, byte(len(context)))
	info = append(info, context...)
	out, err := hkdf.Expand(h, secret, string(info), length)
	if err != nil {
		panic(err)
	}
	return out
}

func hmacSum(h func() hash.Hash, key, data []byte) []byte {
	m := hmac.New(h, key)
	m.Write(data)
	return m.Sum(nil)
}

func recordNonce(iv []byte, seq uint64) []byte {
	nonce := make([]byte, 12)
	copy(nonce, iv)
	var seqBytes [8]byte
	binary.BigEndian.PutUint64(seqBytes[:], seq)
	for i := 0; i < 8; i++ {
		nonce[4+i] ^= seqBytes[i]
	}
	return nonce
}

func sealAesGcmRecord(key, iv []byte, seq uint64, innerType byte, plaintext []byte, padZeros int) []byte {
	block, err := aes.NewCipher(key)
	if err != nil {
		panic(err)
	}
	aead, err := cipher.NewGCM(block)
	if err != nil {
		panic(err)
	}
	inner := make([]byte, 0, len(plaintext)+1+padZeros)
	inner = append(inner, plaintext...)
	inner = append(inner, innerType)
	for i := 0; i < padZeros; i++ {
		inner = append(inner, 0)
	}
	ctLen := len(inner) + aead.Overhead()
	aad := []byte{0x17, 0x03, 0x03, byte(ctLen >> 8), byte(ctLen)}
	nonce := recordNonce(iv, seq)
	ct := aead.Seal(nil, nonce, inner, aad)
	return append(aad, ct...)
}

type keyScheduleVector struct {
	Name                   string `json:"name"`
	Suite                  string `json:"suite"`
	SharedSecretHex        string `json:"shared_secret_hex"`
	ChShHashHex            string `json:"ch_sh_hash_hex"`
	ChSfHashHex            string `json:"ch_sf_hash_hex"`
	ChCfHashHex            string `json:"ch_cf_hash_hex"`
	EarlySecretHex         string `json:"early_secret_hex"`
	DerivedEarlyHex        string `json:"derived_early_hex"`
	HandshakeSecretHex     string `json:"handshake_secret_hex"`
	ClientHsTrafficHex     string `json:"client_hs_traffic_hex"`
	ServerHsTrafficHex     string `json:"server_hs_traffic_hex"`
	ClientHsKeyHex         string `json:"client_hs_key_hex"`
	ClientHsIvHex          string `json:"client_hs_iv_hex"`
	ServerHsKeyHex         string `json:"server_hs_key_hex"`
	ServerHsIvHex          string `json:"server_hs_iv_hex"`
	ServerFinishedKeyHex   string `json:"server_finished_key_hex"`
	ServerVerifyDataHex    string `json:"server_verify_data_hex"`
	DerivedHsHex           string `json:"derived_hs_hex"`
	MasterSecretHex        string `json:"master_secret_hex"`
	ClientApTrafficHex     string `json:"client_ap_traffic_hex"`
	ServerApTrafficHex     string `json:"server_ap_traffic_hex"`
	ClientApKeyHex         string `json:"client_ap_key_hex"`
	ClientApIvHex          string `json:"client_ap_iv_hex"`
	ClientApTrafficNextHex string `json:"client_ap_traffic_next_hex"`
	ExporterMasterHex      string `json:"exporter_master_hex"`
	ExportedMaterialHex    string `json:"exported_material_hex"`
	ResumptionMasterHex    string `json:"resumption_master_hex"`
}

type recordVector struct {
	Name         string `json:"name"`
	Suite        string `json:"suite"`
	KeyHex       string `json:"key_hex"`
	IvHex        string `json:"iv_hex"`
	Seq          uint64 `json:"seq"`
	NonceHex     string `json:"nonce_hex"`
	ContentType  uint8  `json:"content_type"`
	PlaintextHex string `json:"plaintext_hex"`
	PadZeros     int    `json:"pad_zeros"`
	WireHex      string `json:"wire_hex"`
}

type certFixture struct {
	Name           string   `json:"name"`
	KeyType        string   `json:"key_type"`
	ServerName     string   `json:"server_name"`
	NowUnix        int64    `json:"now_unix"`
	RootPem        string   `json:"root_pem"`
	ChainPem       string   `json:"chain_pem"`
	KeyPem         string   `json:"key_pem"`
	ChainDerHex    []string `json:"chain_der_hex"`
	RootDerHex     string   `json:"root_der_hex"`
	KeyPkcs8DerHex string   `json:"key_pkcs8_der_hex"`
}

type vectorBundle struct {
	KeySchedules []keyScheduleVector `json:"key_schedules"`
	Records      []recordVector      `json:"records"`
	Certs        []certFixture       `json:"certs"`
}

func buildKeyScheduleVector(name, suite string, h func() hash.Hash, keyLen int, shared, chSh, chSf, chCf []byte) keyScheduleVector {
	hLen := h().Size()
	zeros := make([]byte, hLen)
	emptyHash := h().Sum(nil)

	early := hkdfExtract(h, zeros, zeros)
	derivedEarly := hkdfExpandLabel(h, early, "derived", emptyHash, hLen)
	hs := hkdfExtract(h, derivedEarly, shared)
	cHs := hkdfExpandLabel(h, hs, "c hs traffic", chSh, hLen)
	sHs := hkdfExpandLabel(h, hs, "s hs traffic", chSh, hLen)
	cHsKey := hkdfExpandLabel(h, cHs, "key", nil, keyLen)
	cHsIv := hkdfExpandLabel(h, cHs, "iv", nil, 12)
	sHsKey := hkdfExpandLabel(h, sHs, "key", nil, keyLen)
	sHsIv := hkdfExpandLabel(h, sHs, "iv", nil, 12)
	sFinKey := hkdfExpandLabel(h, sHs, "finished", nil, hLen)
	sVerify := hmacSum(h, sFinKey, chSf)

	derivedHs := hkdfExpandLabel(h, hs, "derived", emptyHash, hLen)
	master := hkdfExtract(h, derivedHs, zeros)
	cAp := hkdfExpandLabel(h, master, "c ap traffic", chSf, hLen)
	sAp := hkdfExpandLabel(h, master, "s ap traffic", chSf, hLen)
	cApKey := hkdfExpandLabel(h, cAp, "key", nil, keyLen)
	cApIv := hkdfExpandLabel(h, cAp, "iv", nil, 12)
	cApNext := hkdfExpandLabel(h, cAp, "traffic upd", nil, hLen)
	expMaster := hkdfExpandLabel(h, master, "exp master", chSf, hLen)
	resMaster := hkdfExpandLabel(h, master, "res master", chCf, hLen)

	// RFC 8446 §7.5 TLS-Exporter(label, context, length)
	derivedExp := hkdfExpandLabel(h, expMaster, "EXPORTER-cinch-test", emptyHash, hLen)
	ctxHash := h()
	ctxHash.Write([]byte("exporter-context"))
	exported := hkdfExpandLabel(h, derivedExp, "exporter", ctxHash.Sum(nil), 32)

	return keyScheduleVector{
		Name:                   name,
		Suite:                  suite,
		SharedSecretHex:        hex.EncodeToString(shared),
		ChShHashHex:            hex.EncodeToString(chSh),
		ChSfHashHex:            hex.EncodeToString(chSf),
		ChCfHashHex:            hex.EncodeToString(chCf),
		EarlySecretHex:         hex.EncodeToString(early),
		DerivedEarlyHex:        hex.EncodeToString(derivedEarly),
		HandshakeSecretHex:     hex.EncodeToString(hs),
		ClientHsTrafficHex:     hex.EncodeToString(cHs),
		ServerHsTrafficHex:     hex.EncodeToString(sHs),
		ClientHsKeyHex:         hex.EncodeToString(cHsKey),
		ClientHsIvHex:          hex.EncodeToString(cHsIv),
		ServerHsKeyHex:         hex.EncodeToString(sHsKey),
		ServerHsIvHex:          hex.EncodeToString(sHsIv),
		ServerFinishedKeyHex:   hex.EncodeToString(sFinKey),
		ServerVerifyDataHex:    hex.EncodeToString(sVerify),
		DerivedHsHex:           hex.EncodeToString(derivedHs),
		MasterSecretHex:        hex.EncodeToString(master),
		ClientApTrafficHex:     hex.EncodeToString(cAp),
		ServerApTrafficHex:     hex.EncodeToString(sAp),
		ClientApKeyHex:         hex.EncodeToString(cApKey),
		ClientApIvHex:          hex.EncodeToString(cApIv),
		ClientApTrafficNextHex: hex.EncodeToString(cApNext),
		ExporterMasterHex:      hex.EncodeToString(expMaster),
		ExportedMaterialHex:    hex.EncodeToString(exported),
		ResumptionMasterHex:    hex.EncodeToString(resMaster),
	}
}

func mustHex(s string) []byte {
	b, err := hex.DecodeString(s)
	if err != nil {
		panic(err)
	}
	return b
}

func makeKey(kind string, rng io.Reader) (crypto.Signer, crypto.PublicKey) {
	switch kind {
	case "ed25519":
		pub, priv, err := ed25519.GenerateKey(rng)
		if err != nil {
			panic(err)
		}
		return priv, pub
	case "ecdsa":
		priv, err := ecdsa.GenerateKey(elliptic.P256(), rng)
		if err != nil {
			panic(err)
		}
		return priv, &priv.PublicKey
	case "rsa":
		priv, err := rsa.GenerateKey(rng, 2048)
		if err != nil {
			panic(err)
		}
		return priv, &priv.PublicKey
	default:
		panic("unknown key kind: " + kind)
	}
}

func makeCertChain(name, keyType, dnsName string, withIntermediate bool, notBefore, notAfter time.Time, rng io.Reader) certFixture {
	rootPriv, rootPub := makeKey(keyType, rng)
	rootTmpl := &x509.Certificate{
		SerialNumber:          big.NewInt(100),
		Subject:               pkix.Name{CommonName: "Cinch Test Root " + name},
		NotBefore:             notBefore.Add(-time.Hour),
		NotAfter:              notAfter.Add(time.Hour),
		KeyUsage:              x509.KeyUsageCertSign | x509.KeyUsageCRLSign,
		BasicConstraintsValid: true,
		IsCA:                  true,
		MaxPathLen:            2,
		SubjectKeyId:          []byte{1, 2, 3, 4},
	}
	if keyType == "rsa" {
		rootTmpl.SignatureAlgorithm = x509.SHA256WithRSAPSS
	}
	rootDer, err := x509.CreateCertificate(rng, rootTmpl, rootTmpl, rootPub, rootPriv)
	if err != nil {
		panic(err)
	}
	rootCert, err := x509.ParseCertificate(rootDer)
	if err != nil {
		panic(err)
	}

	signerCert := rootCert
	signerPriv := rootPriv
	var chainDers [][]byte

	var interDer []byte
	if withIntermediate {
		interPriv, interPub := makeKey(keyType, rng)
		interTmpl := &x509.Certificate{
			SerialNumber:          big.NewInt(200),
			Subject:               pkix.Name{CommonName: "Cinch Test Intermediate " + name},
			NotBefore:             notBefore.Add(-30 * time.Minute),
			NotAfter:              notAfter.Add(30 * time.Minute),
			KeyUsage:              x509.KeyUsageCertSign | x509.KeyUsageCRLSign,
			BasicConstraintsValid: true,
			IsCA:                  true,
			MaxPathLen:            0,
			MaxPathLenZero:        true,
			SubjectKeyId:          []byte{5, 6, 7, 8},
		}
		if keyType == "rsa" {
			interTmpl.SignatureAlgorithm = x509.SHA256WithRSAPSS
		}
		interDer, err = x509.CreateCertificate(rng, interTmpl, rootCert, interPub, rootPriv)
		if err != nil {
			panic(err)
		}
		interCert, err := x509.ParseCertificate(interDer)
		if err != nil {
			panic(err)
		}
		signerCert = interCert
		signerPriv = interPriv
	}

	leafPriv, leafPub := makeKey(keyType, rng)
	leafTmpl := &x509.Certificate{
		SerialNumber:          big.NewInt(300),
		Subject:               pkix.Name{CommonName: dnsName},
		NotBefore:             notBefore,
		NotAfter:              notAfter,
		KeyUsage:              x509.KeyUsageDigitalSignature,
		ExtKeyUsage:           []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth, x509.ExtKeyUsageClientAuth},
		BasicConstraintsValid: true,
		IsCA:                  false,
		DNSNames:              []string{dnsName, "localhost"},
		IPAddresses:           []net.IP{net.ParseIP("127.0.0.1")},
		SubjectKeyId:          []byte{9, 10, 11, 12},
	}
	if keyType == "rsa" {
		leafTmpl.SignatureAlgorithm = x509.SHA256WithRSAPSS
	}
	leafDer, err := x509.CreateCertificate(rng, leafTmpl, signerCert, leafPub, signerPriv)
	if err != nil {
		panic(err)
	}

	chainDers = append(chainDers, leafDer)
	if withIntermediate {
		chainDers = append(chainDers, interDer)
	}

	rootPem := string(pem.EncodeToMemory(&pem.Block{Type: "CERTIFICATE", Bytes: rootDer}))
	var chainPem strings.Builder
	var chainHex []string
	for _, d := range chainDers {
		chainPem.Write(pem.EncodeToMemory(&pem.Block{Type: "CERTIFICATE", Bytes: d}))
		chainHex = append(chainHex, hex.EncodeToString(d))
	}

	pkcs8, err := x509.MarshalPKCS8PrivateKey(leafPriv)
	if err != nil {
		panic(err)
	}
	keyPem := string(pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: pkcs8}))

	return certFixture{
		Name:           name,
		KeyType:        keyType,
		ServerName:     dnsName,
		NowUnix:        notBefore.Unix() + 3600,
		RootPem:        rootPem,
		ChainPem:       chainPem.String(),
		KeyPem:         keyPem,
		ChainDerHex:    chainHex,
		RootDerHex:     hex.EncodeToString(rootDer),
		KeyPkcs8DerHex: hex.EncodeToString(pkcs8),
	}
}

func generateVectors() {
	// RFC 8448 §3 Simple 1-RTT Handshake shared secret & transcript hashes
	rfcShared := mustHex("8bd4054fb55b9d63fdfbacf9f04b9f0d35e6d63f537563efd46272900f89492d")
	rfcChSh := mustHex("860c06edc07858ee8e78f0e7428c58edd6b43f2ca3e6e95f02ed063cf0e1cad8")
	rfcChSf := mustHex("9608102a0f1ccc6db6250b7b7e417b1a000eaada3daae4777a7686c9ff83df13")
	rfcChCf := mustHex("209145a96ee8e2a122ff810047cc952684658d6049e86429b735b86748cd3315")

	// Cross-check RFC 8448 §3 expected values
	ks128 := buildKeyScheduleVector("rfc8448_aes128_sha256", "TLS_AES_128_GCM_SHA256", sha256.New, 16, rfcShared, rfcChSh, rfcChSf, rfcChCf)
	if ks128.EarlySecretHex != "33ad0a1c607ec03b09e6cd9893680ce210adf300aa1f2660e1b22e10f170f92a" {
		panic("RFC 8448 early secret mismatch: " + ks128.EarlySecretHex)
	}
	if ks128.HandshakeSecretHex != "1dc826e93606aa6fdc0aadc12f741b01046aa6b99f691ed221a9f0ca043fbeac" {
		panic("RFC 8448 handshake secret mismatch: " + ks128.HandshakeSecretHex)
	}
	if ks128.ClientHsTrafficHex != "b3eddb126e067f35a780b3abf45e2d8f3b1a950738f52e9600746a0e27a55a21" {
		panic("RFC 8448 c_hs_traffic mismatch: " + ks128.ClientHsTrafficHex)
	}
	if ks128.ServerHsTrafficHex != "b67b7d690cc16c4e75e54213cb2d37b4e9c912bcded9105d42befd59d391ad38" {
		panic("RFC 8448 s_hs_traffic mismatch: " + ks128.ServerHsTrafficHex)
	}
	if ks128.ServerHsKeyHex != "3fce516009c21727d0f2e4e86ee403bc" {
		panic("RFC 8448 s_hs_key mismatch: " + ks128.ServerHsKeyHex)
	}
	if ks128.ServerHsIvHex != "5d313eb2671276ee13000b30" {
		panic("RFC 8448 s_hs_iv mismatch: " + ks128.ServerHsIvHex)
	}
	if ks128.MasterSecretHex != "18df06843d13a08bf2a449844c5f8a478001bc4d4c627984d5a41da8d0402919" {
		panic("RFC 8448 master secret mismatch: " + ks128.MasterSecretHex)
	}

	// SHA-384 (TLS_AES_256_GCM_SHA384) key schedule vector
	h384Shared := sha512.Sum384([]byte("cinch-tls13-sha384-shared"))
	h384ChSh := sha512.Sum384([]byte("cinch-tls13-sha384-ch-sh"))
	h384ChSf := sha512.Sum384([]byte("cinch-tls13-sha384-ch-sf"))
	h384ChCf := sha512.Sum384([]byte("cinch-tls13-sha384-ch-cf"))
	ks256 := buildKeyScheduleVector("sha384_aes256_gcm", "TLS_AES_256_GCM_SHA384", sha512.New384, 32, h384Shared[:32], h384ChSh[:], h384ChSf[:], h384ChCf[:])

	// Record seal/open vectors (seq=0, seq=1, seq=0x0102030405, plus zero-padded inner plaintext!)
	k128 := mustHex(ks128.ServerHsKeyHex)
	iv128 := mustHex(ks128.ServerHsIvHex)
	k256 := mustHex(ks256.ClientApKeyHex)
	iv256 := mustHex(ks256.ClientApIvHex)

	pt1 := []byte("Hello TLS 1.3 from Go 1.24!")
	pt2 := []byte("Second record with padding")
	pt3 := []byte("AES-256-GCM-SHA384 record payload")

	records := []recordVector{
		{
			Name:         "aes128_seq0_appdata",
			Suite:        "TLS_AES_128_GCM_SHA256",
			KeyHex:       hex.EncodeToString(k128),
			IvHex:        hex.EncodeToString(iv128),
			Seq:          0,
			NonceHex:     hex.EncodeToString(recordNonce(iv128, 0)),
			ContentType:  23,
			PlaintextHex: hex.EncodeToString(pt1),
			PadZeros:     0,
			WireHex:      hex.EncodeToString(sealAesGcmRecord(k128, iv128, 0, 23, pt1, 0)),
		},
		{
			Name:         "aes128_seq42_padded_handshake",
			Suite:        "TLS_AES_128_GCM_SHA256",
			KeyHex:       hex.EncodeToString(k128),
			IvHex:        hex.EncodeToString(iv128),
			Seq:          42,
			NonceHex:     hex.EncodeToString(recordNonce(iv128, 42)),
			ContentType:  22,
			PlaintextHex: hex.EncodeToString(pt2),
			PadZeros:     7,
			WireHex:      hex.EncodeToString(sealAesGcmRecord(k128, iv128, 42, 22, pt2, 7)),
		},
		{
			Name:         "aes256_seq123456_appdata",
			Suite:        "TLS_AES_256_GCM_SHA384",
			KeyHex:       hex.EncodeToString(k256),
			IvHex:        hex.EncodeToString(iv256),
			Seq:          123456,
			NonceHex:     hex.EncodeToString(recordNonce(iv256, 123456)),
			ContentType:  23,
			PlaintextHex: hex.EncodeToString(pt3),
			PadZeros:     3,
			WireHex:      hex.EncodeToString(sealAesGcmRecord(k256, iv256, 123456, 23, pt3, 3)),
		},
	}

	baseTime := time.Unix(1700000000, 0).UTC()
	certs := []certFixture{
		makeCertChain("ed25519", "ed25519", "ed25519.test", false, baseTime, baseTime.Add(24*time.Hour), newDeterministicReader("ed25519-chain")),
		makeCertChain("ecdsa_p256_chain", "ecdsa", "ecdsa.test", true, baseTime, baseTime.Add(24*time.Hour), newDeterministicReader("ecdsa-chain")),
		makeCertChain("rsa_pss", "rsa", "rsa.test", false, baseTime, baseTime.Add(24*time.Hour), newDeterministicReader("rsa-chain")),
	}

	bundle := vectorBundle{
		KeySchedules: []keyScheduleVector{ks128, ks256},
		Records:      records,
		Certs:        certs,
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(bundle); err != nil {
		panic(err)
	}
}

func runLiveServer(certType, alpnCSV, caOutPath string) {
	now := time.Now().UTC()
	fix := makeCertChain("live-srv", certType, "server.test", certType == "ecdsa", now.Add(-time.Hour), now.Add(24*time.Hour), rand.Reader)
	if caOutPath != "" {
		if err := os.WriteFile(caOutPath, []byte(fix.RootPem), 0600); err != nil {
			panic(err)
		}
	}

	tlsCert, err := tls.X509KeyPair([]byte(fix.ChainPem), []byte(fix.KeyPem))
	if err != nil {
		panic(err)
	}

	var nextProtos []string
	if alpnCSV != "" {
		for _, p := range strings.Split(alpnCSV, ",") {
			if p != "" {
				nextProtos = append(nextProtos, p)
			}
		}
	}

	cfg := &tls.Config{
		MinVersion:   tls.VersionTLS13,
		MaxVersion:   tls.VersionTLS13,
		Certificates: []tls.Certificate{tlsCert},
		NextProtos:   nextProtos,
	}

	ln, err := tls.Listen("tcp", "127.0.0.1:0", cfg)
	if err != nil {
		panic(err)
	}
	defer ln.Close()

	_, port, err := net.SplitHostPort(ln.Addr().String())
	if err != nil {
		panic(err)
	}
	fmt.Printf("PORT=%s\n", port)
	os.Stdout.Sync()

	conn, err := ln.Accept()
	if err != nil {
		fmt.Fprintf(os.Stderr, "accept error: %v\n", err)
		os.Exit(2)
	}
	defer conn.Close()

	tlsConn := conn.(*tls.Conn)
	_ = tlsConn.SetDeadline(time.Now().Add(15 * time.Second))
	if err := tlsConn.Handshake(); err != nil {
		fmt.Fprintf(os.Stderr, "server handshake error: %v\n", err)
		os.Exit(3)
	}
	st := tlsConn.ConnectionState()
	if st.Version != tls.VersionTLS13 {
		fmt.Fprintf(os.Stderr, "expected TLS 1.3, got %x\n", st.Version)
		os.Exit(4)
	}

	buf := make([]byte, 4096)
	n, err := tlsConn.Read(buf)
	if err != nil && err != io.EOF {
		fmt.Fprintf(os.Stderr, "server read error: %v\n", err)
		os.Exit(5)
	}
	msg := string(buf[:n])
	reply := fmt.Sprintf("GO-SRV-ACK:%s|sni=%s|alpn=%s|suite=%s",
		msg, st.ServerName, st.NegotiatedProtocol, tls.CipherSuiteName(st.CipherSuite))
	if _, err := tlsConn.Write([]byte(reply)); err != nil {
		fmt.Fprintf(os.Stderr, "server write error: %v\n", err)
		os.Exit(6)
	}
	// Drain until peer closes cleanly (close_notify)
	_, _ = tlsConn.Read(buf)
	_ = tlsConn.Close()
}

func runLiveClient(certType, sni, alpnCSV, curveName, rootOut, chainOut, keyOut string) {
	now := time.Now().UTC()
	fix := makeCertChain("live-cli", certType, sni, certType == "ecdsa", now.Add(-time.Hour), now.Add(24*time.Hour), rand.Reader)
	if rootOut != "" {
		if err := os.WriteFile(rootOut, []byte(fix.RootPem), 0600); err != nil {
			panic(err)
		}
	}
	if chainOut != "" {
		if err := os.WriteFile(chainOut, []byte(fix.ChainPem), 0600); err != nil {
			panic(err)
		}
	}
	if keyOut != "" {
		if err := os.WriteFile(keyOut, []byte(fix.KeyPem), 0600); err != nil {
			panic(err)
		}
	}

	pool := x509.NewCertPool()
	if !pool.AppendCertsFromPEM([]byte(fix.RootPem)) {
		panic("failed to load root PEM")
	}

	var nextProtos []string
	if alpnCSV != "" {
		for _, p := range strings.Split(alpnCSV, ",") {
			if p != "" {
				nextProtos = append(nextProtos, p)
			}
		}
	}

	var curves []tls.CurveID
	switch curveName {
	case "x25519":
		curves = []tls.CurveID{tls.X25519}
	case "p256":
		curves = []tls.CurveID{tls.CurveP256}
	}

	// Listen on raw TCP so Cinch (which has client TCP connect) connects to us,
	// then run Go's tls.Client over the accepted TCP stream while Cinch runs tls-server!
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		panic(err)
	}
	defer ln.Close()

	_, port, err := net.SplitHostPort(ln.Addr().String())
	if err != nil {
		panic(err)
	}
	fmt.Printf("PORT=%s\n", port)
	os.Stdout.Sync()

	rawConn, err := ln.Accept()
	if err != nil {
		fmt.Fprintf(os.Stderr, "tcp accept error: %v\n", err)
		os.Exit(2)
	}
	defer rawConn.Close()
	_ = rawConn.SetDeadline(time.Now().Add(15 * time.Second))

	cfg := &tls.Config{
		MinVersion:       tls.VersionTLS13,
		MaxVersion:       tls.VersionTLS13,
		RootCAs:          pool,
		ServerName:       sni,
		NextProtos:       nextProtos,
		CurvePreferences: curves,
	}
	tlsConn := tls.Client(rawConn, cfg)
	if err := tlsConn.Handshake(); err != nil {
		fmt.Fprintf(os.Stderr, "go client handshake error: %v\n", err)
		os.Exit(3)
	}
	st := tlsConn.ConnectionState()
	if st.Version != tls.VersionTLS13 {
		fmt.Fprintf(os.Stderr, "expected TLS 1.3, got %x\n", st.Version)
		os.Exit(4)
	}

	if _, err := tlsConn.Write([]byte("HELLO-FROM-GO-CLIENT")); err != nil {
		fmt.Fprintf(os.Stderr, "go client write error: %v\n", err)
		os.Exit(5)
	}

	buf := make([]byte, 4096)
	n, err := tlsConn.Read(buf)
	if err != nil && err != io.EOF {
		fmt.Fprintf(os.Stderr, "go client read error: %v\n", err)
		os.Exit(6)
	}
	got := string(buf[:n])
	wantPrefix := "CINCH-SRV-ACK:HELLO-FROM-GO-CLIENT"
	if !strings.HasPrefix(got, wantPrefix) {
		fmt.Fprintf(os.Stderr, "unexpected reply %q (want prefix %q)\n", got, wantPrefix)
		os.Exit(7)
	}
	fmt.Printf("CLIENT_OK=%s|alpn=%s|suite=%s\n", got, st.NegotiatedProtocol, tls.CipherSuiteName(st.CipherSuite))
	os.Stdout.Sync()
	_ = tlsConn.Close()
}

func main() {
	mode := flag.String("mode", "vectors", "vectors | server | client")
	certType := flag.String("cert-type", "ed25519", "ed25519 | ecdsa | rsa")
	alpn := flag.String("alpn", "", "comma-separated ALPN protocols")
	sni := flag.String("sni", "server.test", "SNI hostname for client mode")
	curve := flag.String("curve", "", "curve preference for client mode: x25519 | p256 | empty(default)")
	caOut := flag.String("ca-out", "", "path to write root CA PEM")
	chainOut := flag.String("chain-out", "", "path to write server cert chain PEM")
	keyOut := flag.String("key-out", "", "path to write server private key PEM")
	flag.Parse()

	switch *mode {
	case "vectors":
		generateVectors()
	case "server":
		runLiveServer(*certType, *alpn, *caOut)
	case "client":
		runLiveClient(*certType, *sni, *alpn, *curve, *caOut, *chainOut, *keyOut)
	default:
		fmt.Fprintf(os.Stderr, "unknown mode: %s\n", *mode)
		os.Exit(1)
	}
}
