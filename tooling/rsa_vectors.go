// Generate independent Go 1.24 crypto/rsa + crypto/x509 PKCS#1 reference vectors.
// Run: GOCACHE=/tmp/go-cache GOTELEMETRY=off go run tooling/rsa_vectors.go > seed/tests/fixtures/rsa-vectors.json
package main

import (
	"bytes"
	"crypto"
	"crypto/md5"
	"crypto/rsa"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/sha512"
	"crypto/x509"
	"encoding/asn1"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"hash"
	"math/big"
	"os"
)

// detReader is a deterministic SHA-256 counter-mode byte stream for reproducible fixtures.
type detReader struct {
	seed    []byte
	counter uint64
	buf     []byte
}

func newDetReader(label string) *detReader {
	h := sha256.Sum256([]byte(label))
	return &detReader{seed: h[:]}
}

func (r *detReader) Read(p []byte) (int, error) {
	n := 0
	for n < len(p) {
		if len(r.buf) == 0 {
			var in [40]byte
			copy(in[:32], r.seed)
			c := r.counter
			for i := 7; i >= 0; i-- {
				in[32+i] = byte(c)
				c >>= 8
			}
			r.counter++
			h := sha256.Sum256(in[:])
			r.buf = h[:]
		}
		k := copy(p[n:], r.buf)
		r.buf = r.buf[k:]
		n += k
	}
	return n, nil
}

type KeyVector struct {
	Name            string `json:"name"`
	Bits            int    `json:"bits"`
	Size            int    `json:"size"`
	N               string `json:"n"`
	E               int    `json:"e"`
	D               string `json:"d"`
	P               string `json:"p"`
	Q               string `json:"q"`
	Dp              string `json:"dp"`
	Dq              string `json:"dq"`
	Qinv            string `json:"qinv"`
	Pkcs1PublicDer  string `json:"pkcs1_public_der"`
	Pkcs1PrivateDer string `json:"pkcs1_private_der"`
}

type Pkcs1v15SignVector struct {
	Key    string `json:"key"`
	Hash   string `json:"hash"`
	Digest string `json:"digest"`
	Sig    string `json:"sig"`
}

type PssVector struct {
	Key        string `json:"key"`
	Hash       string `json:"hash"`
	Digest     string `json:"digest"`
	SaltPolicy string `json:"salt_policy"`
	SaltLen    int    `json:"salt_len"`
	Salt       string `json:"salt"`
	Sig        string `json:"sig"`
}

type OaepVector struct {
	Key        string `json:"key"`
	Hash       string `json:"hash"`
	Label      string `json:"label"`
	Message    string `json:"message"`
	Seed       string `json:"seed"`
	Ciphertext string `json:"ciphertext"`
}

type Pkcs1v15EncVector struct {
	Key        string `json:"key"`
	Message    string `json:"message"`
	Pad        string `json:"pad"`
	Ciphertext string `json:"ciphertext"`
}

type BadSigVector struct {
	Key    string `json:"key"`
	Kind   string `json:"kind"`
	Hash   string `json:"hash"`
	Digest string `json:"digest"`
	Sig    string `json:"sig"`
}

type BadEncVector struct {
	Key        string `json:"key"`
	Kind       string `json:"kind"`
	Hash       string `json:"hash"`
	Label      string `json:"label"`
	Ciphertext string `json:"ciphertext"`
}

type Fixture struct {
	Keys            []KeyVector         `json:"keys"`
	Pkcs1v15Signs   []Pkcs1v15SignVector `json:"pkcs1v15_signs"`
	PssSigns        []PssVector         `json:"pss_signs"`
	OaepEncs        []OaepVector        `json:"oaep_encs"`
	Pkcs1v15Encs    []Pkcs1v15EncVector `json:"pkcs1v15_encs"`
	BadSigs         []BadSigVector      `json:"bad_sigs"`
	BadEncs         []BadEncVector      `json:"bad_encs"`
	BadPublicDers   []string            `json:"bad_public_ders"`
	BadPrivateDers  []string            `json:"bad_private_ders"`
}

func hexOf(b []byte) string {
	return hex.EncodeToString(b)
}

func bigHex(n *big.Int) string {
	return hex.EncodeToString(n.Bytes())
}

func hashByName(name string) (crypto.Hash, func() hash.Hash) {
	switch name {
	case "md5":
		return crypto.MD5, md5.New
	case "sha1":
		return crypto.SHA1, sha1.New
	case "sha224":
		return crypto.SHA224, sha256.New224
	case "sha256":
		return crypto.SHA256, sha256.New
	case "sha384":
		return crypto.SHA384, sha512.New384
	case "sha512":
		return crypto.SHA512, sha512.New
	case "sha512_224":
		return crypto.SHA512_224, sha512.New512_224
	case "sha512_256":
		return crypto.SHA512_256, sha512.New512_256
	default:
		panic("unknown hash: " + name)
	}
}

func digestOf(name string, msg []byte) []byte {
	if name == "none" {
		m := md5.Sum(msg)
		s := sha1.Sum(msg)
		return append(m[:], s[:]...)
	}
	_, fn := hashByName(name)
	h := fn()
	h.Write(msg)
	return h.Sum(nil)
}

func mgf1XOR(out []byte, h hash.Hash, seed []byte) {
	var counter [4]byte
	var digest []byte
	done := 0
	for done < len(out) {
		h.Reset()
		h.Write(seed)
		h.Write(counter[:])
		digest = h.Sum(digest[:0])
		for i := 0; i < len(digest) && done < len(out); i++ {
			out[done] ^= digest[i]
			done++
		}
		for i := 3; i >= 0; i-- {
			counter[i]++
			if counter[i] != 0 {
				break
			}
		}
	}
}

// Recover exact salt used in a valid Go PSS signature by public-key exponentiation.
func extractPssSalt(pub *rsa.PublicKey, h hash.Hash, sig []byte) []byte {
	s := new(big.Int).SetBytes(sig)
	e := big.NewInt(int64(pub.E))
	m := new(big.Int).Exp(s, e, pub.N)
	emBits := pub.N.BitLen() - 1
	emLen := (emBits + 7) / 8
	em := make([]byte, emLen)
	m.FillBytes(em)
	hLen := h.Size()
	maskedDB := append([]byte(nil), em[:emLen-hLen-1]...)
	H := em[emLen-hLen-1 : emLen-1]
	mgf1XOR(maskedDB, h, H)
	maskedDB[0] &= 0xff >> uint(8*emLen-emBits)
	idx := bytes.IndexByte(maskedDB, 0x01)
	if idx < 0 {
		panic("failed to extract PSS salt")
	}
	return maskedDB[idx+1:]
}

// Recover exact OAEP seed used in a valid Go OAEP ciphertext via private-key exponentiation.
func extractOaepSeed(priv *rsa.PrivateKey, h hash.Hash, ct []byte) []byte {
	c := new(big.Int).SetBytes(ct)
	m := new(big.Int).Exp(c, priv.D, priv.N)
	k := priv.Size()
	em := make([]byte, k)
	m.FillBytes(em)
	hLen := h.Size()
	seed := append([]byte(nil), em[1:1+hLen]...)
	maskedDB := em[1+hLen:]
	mgf1XOR(seed, h, maskedDB)
	return seed
}

// Recover exact PKCS#1 v1.5 non-zero padding bytes from a Go ciphertext.
func extractPkcs1v15Pad(priv *rsa.PrivateKey, ct []byte) []byte {
	c := new(big.Int).SetBytes(ct)
	m := new(big.Int).Exp(c, priv.D, priv.N)
	k := priv.Size()
	em := make([]byte, k)
	m.FillBytes(em)
	if em[0] != 0x00 || em[1] != 0x02 {
		panic("bad pkcs1v15 em header")
	}
	idx := bytes.IndexByte(em[2:], 0x00)
	if idx < 8 {
		panic("bad pkcs1v15 em separator")
	}
	return em[2 : 2+idx]
}

func flipByte(b []byte, idx int) []byte {
	out := append([]byte(nil), b...)
	out[idx] ^= 0x01
	return out
}

func must(err error) {
	if err != nil {
		panic(err)
	}
}

func main() {
	rng := newDetReader("cinch-crypto-rsa-vectors-v1")

	key1024, err := rsa.GenerateKey(rng, 1024)
	must(err)
	key1024.Precompute()
	must(key1024.Validate())

	key2048, err := rsa.GenerateKey(rng, 2048)
	must(err)
	key2048.Precompute()
	must(key2048.Validate())

	keyMap := []struct {
		name string
		key  *rsa.PrivateKey
	}{
		{"rsa1024", key1024},
		{"rsa2048", key2048},
	}

	var fix Fixture

	for _, km := range keyMap {
		k := km.key
		pubDer := x509.MarshalPKCS1PublicKey(&k.PublicKey)
		privDer := x509.MarshalPKCS1PrivateKey(k)
		// Sanity roundtrip in Go x509.
		_, err := x509.ParsePKCS1PublicKey(pubDer)
		must(err)
		_, err = x509.ParsePKCS1PrivateKey(privDer)
		must(err)

		fix.Keys = append(fix.Keys, KeyVector{
			Name:            km.name,
			Bits:            k.N.BitLen(),
			Size:            k.Size(),
			N:               bigHex(k.N),
			E:               k.E,
			D:               bigHex(k.D),
			P:               bigHex(k.Primes[0]),
			Q:               bigHex(k.Primes[1]),
			Dp:              bigHex(k.Precomputed.Dp),
			Dq:              bigHex(k.Precomputed.Dq),
			Qinv:            bigHex(k.Precomputed.Qinv),
			Pkcs1PublicDer:  hexOf(pubDer),
			Pkcs1PrivateDer: hexOf(privDer),
		})
	}

	// 1. PKCS#1 v1.5 signatures across hashes on both 1024-bit and 2048-bit keys.
	pkcsHashes := []string{"sha256", "sha384", "sha512", "sha224", "sha1", "md5", "sha512_224", "sha512_256", "none"}
	msg := []byte("Cinch RSA PKCS#1 v1.5 reference test message")
	for _, km := range keyMap {
		for _, hn := range pkcsHashes {
			d := digestOf(hn, msg)
			var ch crypto.Hash
			if hn != "none" {
				ch, _ = hashByName(hn)
			}
			sig, err := rsa.SignPKCS1v15(nil, km.key, ch, d)
			must(err)
			must(rsa.VerifyPKCS1v15(&km.key.PublicKey, ch, d, sig))
			fix.Pkcs1v15Signs = append(fix.Pkcs1v15Signs, Pkcs1v15SignVector{
				Key:    km.name,
				Hash:   hn,
				Digest: hexOf(d),
				Sig:    hexOf(sig),
			})
		}
	}

	// 2. PSS signatures across SHA-256, SHA-384, SHA-512, SHA-224, SHA-1 and salt policies.
	pssSpecs := []struct {
		keyIdx int
		hash   string
		policy string
		optLen int
	}{
		{0, "sha256", "equals_hash", rsa.PSSSaltLengthEqualsHash},
		{0, "sha256", "auto", rsa.PSSSaltLengthAuto},
		{0, "sha256", "exact", 0},
		{0, "sha256", "exact", 20},
		{0, "sha384", "equals_hash", rsa.PSSSaltLengthEqualsHash},
		{0, "sha512", "auto", rsa.PSSSaltLengthAuto},
		{0, "sha1", "equals_hash", rsa.PSSSaltLengthEqualsHash},
		{1, "sha256", "equals_hash", rsa.PSSSaltLengthEqualsHash},
		{1, "sha256", "auto", rsa.PSSSaltLengthAuto},
		{1, "sha256", "exact", 16},
		{1, "sha384", "equals_hash", rsa.PSSSaltLengthEqualsHash},
		{1, "sha384", "auto", rsa.PSSSaltLengthAuto},
		{1, "sha512", "equals_hash", rsa.PSSSaltLengthEqualsHash},
		{1, "sha512", "auto", rsa.PSSSaltLengthAuto},
		{1, "sha224", "equals_hash", rsa.PSSSaltLengthEqualsHash},
	}
	pssMsg := []byte("Cinch RSASSA-PSS reference vector message")
	for _, spec := range pssSpecs {
		km := keyMap[spec.keyIdx]
		ch, hfn := hashByName(spec.hash)
		d := digestOf(spec.hash, pssMsg)
		opts := &rsa.PSSOptions{SaltLength: spec.optLen, Hash: ch}
		sig, err := rsa.SignPSS(rng, km.key, ch, d, opts)
		must(err)
		must(rsa.VerifyPSS(&km.key.PublicKey, ch, d, sig, opts))
		must(rsa.VerifyPSS(&km.key.PublicKey, ch, d, sig, &rsa.PSSOptions{SaltLength: rsa.PSSSaltLengthAuto, Hash: ch}))
		salt := extractPssSalt(&km.key.PublicKey, hfn(), sig)
		fix.PssSigns = append(fix.PssSigns, PssVector{
			Key:        km.name,
			Hash:       spec.hash,
			Digest:     hexOf(d),
			SaltPolicy: spec.policy,
			SaltLen:    len(salt),
			Salt:       hexOf(salt),
			Sig:        hexOf(sig),
		})
	}

	// 3. OAEP encryption/decryption with labels across SHA-256, SHA-384, SHA-512, SHA-1.
	oaepSpecs := []struct {
		keyIdx int
		hash   string
		label  []byte
		msg    []byte
	}{
		{0, "sha256", []byte{}, []byte("hello rsa1024 oaep")},
		{0, "sha256", []byte("cinch-oaep-label"), []byte{}},
		{0, "sha256", []byte{0x00, 0x01, 0x02, 0xff}, bytes.Repeat([]byte("A"), 128-2*32-2)}, // max len for 1024+sha256
		{0, "sha384", []byte("lbl384"), []byte("short 384")},
		{0, "sha1", []byte("legacy-sha1"), []byte("oaep sha1 message")},
		{1, "sha256", []byte{}, []byte("hello rsa2048 oaep")},
		{1, "sha256", []byte("label-2048"), bytes.Repeat([]byte("B"), 256-2*32-2)}, // max len for 2048+sha256
		{1, "sha384", []byte("label-384"), []byte("rsa2048 oaep sha384 payload")},
		{1, "sha512", []byte("label-512"), []byte("rsa2048 oaep sha512 payload")},
		{1, "sha512", []byte{}, bytes.Repeat([]byte("Z"), 256-2*64-2)}, // max len for 2048+sha512
	}
	for _, spec := range oaepSpecs {
		km := keyMap[spec.keyIdx]
		_, hfn := hashByName(spec.hash)
		ct, err := rsa.EncryptOAEP(hfn(), rng, &km.key.PublicKey, spec.msg, spec.label)
		must(err)
		pt, err := rsa.DecryptOAEP(hfn(), nil, km.key, ct, spec.label)
		must(err)
		if !bytes.Equal(pt, spec.msg) {
			panic("oaep roundtrip mismatch")
		}
		seed := extractOaepSeed(km.key, hfn(), ct)
		fix.OaepEncs = append(fix.OaepEncs, OaepVector{
			Key:        km.name,
			Hash:       spec.hash,
			Label:      hexOf(spec.label),
			Message:    hexOf(spec.msg),
			Seed:       hexOf(seed),
			Ciphertext: hexOf(ct),
		})
	}

	// 4. PKCS#1 v1.5 encryption/decryption.
	pkcsEncSpecs := []struct {
		keyIdx int
		msg    []byte
	}{
		{0, []byte{}},
		{0, []byte("hello pkcs1v15 1024")},
		{0, bytes.Repeat([]byte{0x42}, 128-11)}, // max len for 1024
		{1, []byte("hello pkcs1v15 2048")},
		{1, bytes.Repeat([]byte{0xab}, 32)},     // 32-byte session key
		{1, bytes.Repeat([]byte{0x7f}, 256-11)}, // max len for 2048
	}
	for _, spec := range pkcsEncSpecs {
		km := keyMap[spec.keyIdx]
		ct, err := rsa.EncryptPKCS1v15(rng, &km.key.PublicKey, spec.msg)
		must(err)
		pt, err := rsa.DecryptPKCS1v15(nil, km.key, ct)
		must(err)
		if !bytes.Equal(pt, spec.msg) {
			panic("pkcs1v15 roundtrip mismatch")
		}
		pad := extractPkcs1v15Pad(km.key, ct)
		fix.Pkcs1v15Encs = append(fix.Pkcs1v15Encs, Pkcs1v15EncVector{
			Key:        km.name,
			Message:    hexOf(spec.msg),
			Pad:        hexOf(pad),
			Ciphertext: hexOf(ct),
		})
	}

	// 5. Invalid / tampered signatures.
	sig1024, _ := hex.DecodeString(fix.Pkcs1v15Signs[0].Sig)
	dig1024, _ := hex.DecodeString(fix.Pkcs1v15Signs[0].Digest)
	pss1024, _ := hex.DecodeString(fix.PssSigns[0].Sig)
	pssDig1024, _ := hex.DecodeString(fix.PssSigns[0].Digest)
	sig2048, _ := hex.DecodeString(fix.Pkcs1v15Signs[len(pkcsHashes)].Sig)
	dig2048, _ := hex.DecodeString(fix.Pkcs1v15Signs[len(pkcsHashes)].Digest)
	pss2048, _ := hex.DecodeString(fix.PssSigns[7].Sig)
	pssDig2048, _ := hex.DecodeString(fix.PssSigns[7].Digest)

	nPlus1 := new(big.Int).Add(key1024.N, big.NewInt(1)).Bytes()

	fix.BadSigs = []BadSigVector{
		{"rsa1024", "pkcs1v15", "sha256", hexOf(dig1024), hexOf(flipByte(sig1024, 0))},
		{"rsa1024", "pkcs1v15", "sha256", hexOf(dig1024), hexOf(flipByte(sig1024, len(sig1024)-1))},
		{"rsa1024", "pkcs1v15", "sha256", hexOf(flipByte(dig1024, 0)), hexOf(sig1024)},
		{"rsa1024", "pkcs1v15", "sha256", hexOf(dig1024), hexOf(sig1024[:len(sig1024)-1])},
		{"rsa1024", "pkcs1v15", "sha256", hexOf(dig1024), hexOf(nPlus1)},
		{"rsa2048", "pkcs1v15", "sha256", hexOf(dig2048), hexOf(flipByte(sig2048, 128))},
		{"rsa1024", "pss", "sha256", hexOf(pssDig1024), hexOf(flipByte(pss1024, 0))},
		{"rsa1024", "pss", "sha256", hexOf(pssDig1024), hexOf(flipByte(pss1024, len(pss1024)-1))},
		{"rsa1024", "pss", "sha256", hexOf(flipByte(pssDig1024, 5)), hexOf(pss1024)},
		{"rsa1024", "pss", "sha256", hexOf(pssDig1024), hexOf(pss1024[:len(pss1024)-1])},
		{"rsa2048", "pss", "sha256", hexOf(pssDig2048), hexOf(flipByte(pss2048, 200))},
	}

	// 6. Invalid / tampered ciphertexts (OAEP and PKCS#1 v1.5).
	oaepCt0, _ := hex.DecodeString(fix.OaepEncs[0].Ciphertext)
	oaepCt2048, _ := hex.DecodeString(fix.OaepEncs[5].Ciphertext)
	pkcsCt0, _ := hex.DecodeString(fix.Pkcs1v15Encs[1].Ciphertext)
	pkcsCt2048, _ := hex.DecodeString(fix.Pkcs1v15Encs[3].Ciphertext)

	// Craft a raw RSA encryption of a malformed PKCS#1 v1.5 block (short PS < 8 bytes).
	badEm := make([]byte, key1024.Size())
	badEm[0] = 0x00
	badEm[1] = 0x02
	for i := 2; i < 9; i++ {
		badEm[i] = 0xaa // only 7 pad bytes before 0x00
	}
	badEm[9] = 0x00
	copy(badEm[10:], []byte("too-short-pad"))
	badCtInt := new(big.Int).Exp(new(big.Int).SetBytes(badEm), big.NewInt(int64(key1024.E)), key1024.N)
	badCtBytes := make([]byte, key1024.Size())
	badCtInt.FillBytes(badCtBytes)

	// Craft a raw RSA encryption with wrong block type 0x01 instead of 0x02.
	badEm2 := make([]byte, key1024.Size())
	badEm2[0] = 0x00
	badEm2[1] = 0x01
	for i := 2; i < 20; i++ {
		badEm2[i] = 0xff
	}
	badEm2[20] = 0x00
	badCtInt2 := new(big.Int).Exp(new(big.Int).SetBytes(badEm2), big.NewInt(int64(key1024.E)), key1024.N)
	badCtBytes2 := make([]byte, key1024.Size())
	badCtInt2.FillBytes(badCtBytes2)

	fix.BadEncs = []BadEncVector{
		{"rsa1024", "oaep", "sha256", "", hexOf(flipByte(oaepCt0, 0))},
		{"rsa1024", "oaep", "sha256", hexOf([]byte("wrong-label")), hexOf(oaepCt0)},
		{"rsa1024", "oaep", "sha384", "", hexOf(oaepCt0)},
		{"rsa1024", "oaep", "sha256", "", hexOf(oaepCt0[:len(oaepCt0)-1])},
		{"rsa1024", "oaep", "sha256", "", hexOf(nPlus1)},
		{"rsa2048", "oaep", "sha256", "", hexOf(flipByte(oaepCt2048, 100))},
		{"rsa1024", "pkcs1v15", "", "", hexOf(flipByte(pkcsCt0, 0))},
		{"rsa1024", "pkcs1v15", "", "", hexOf(pkcsCt0[:len(pkcsCt0)-1])},
		{"rsa1024", "pkcs1v15", "", "", hexOf(nPlus1)},
		{"rsa1024", "pkcs1v15", "", "", hexOf(badCtBytes)},
		{"rsa1024", "pkcs1v15", "", "", hexOf(badCtBytes2)},
		{"rsa2048", "pkcs1v15", "", "", hexOf(flipByte(pkcsCt2048, 50))},
	}

	// 7. Malformed PKCS#1 DER keys.
	validPubDer, _ := hex.DecodeString(fix.Keys[0].Pkcs1PublicDer)
	validPrivDer, _ := hex.DecodeString(fix.Keys[0].Pkcs1PrivateDer)

	evenPubDer, _ := asn1.Marshal(struct {
		N *big.Int
		E int
	}{big.NewInt(1000000), 65537})
	badExpPubDer, _ := asn1.Marshal(struct {
		N *big.Int
		E int
	}{key1024.N, 2})
	negModPubDer, _ := asn1.Marshal(struct {
		N *big.Int
		E int
	}{big.NewInt(-17), 65537})
	badVerPrivDer, _ := asn1.Marshal(struct {
		Version int
		N       *big.Int
		E       int
		D       *big.Int
		P       *big.Int
		Q       *big.Int
		Dp      *big.Int
		Dq      *big.Int
		Qinv    *big.Int
	}{1, key1024.N, key1024.E, key1024.D, key1024.Primes[0], key1024.Primes[1], key1024.Precomputed.Dp, key1024.Precomputed.Dq, key1024.Precomputed.Qinv})
	badDPrivDer, _ := asn1.Marshal(struct {
		Version int
		N       *big.Int
		E       int
		D       *big.Int
		P       *big.Int
		Q       *big.Int
		Dp      *big.Int
		Dq      *big.Int
		Qinv    *big.Int
	}{0, key1024.N, key1024.E, new(big.Int).Add(key1024.D, big.NewInt(2)), key1024.Primes[0], key1024.Primes[1], key1024.Precomputed.Dp, key1024.Precomputed.Dq, key1024.Precomputed.Qinv})

	fix.BadPublicDers = []string{
		"",
		hexOf(validPubDer[:len(validPubDer)-1]),
		hexOf(append(append([]byte(nil), validPubDer...), 0x00)),
		hexOf(evenPubDer),
		hexOf(badExpPubDer),
		hexOf(negModPubDer),
	}
	fix.BadPrivateDers = []string{
		"",
		hexOf(validPrivDer[:len(validPrivDer)-1]),
		hexOf(append(append([]byte(nil), validPrivDer...), 0x00)),
		hexOf(validPubDer),
		hexOf(badVerPrivDer),
		hexOf(badDPrivDer),
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(fix); err != nil {
		fmt.Fprintf(os.Stderr, "encode error: %v\n", err)
		os.Exit(1)
	}
}
