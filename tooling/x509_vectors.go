// Generate independent Go 1.24 crypto/x509 + crypto/x509/pkix reference vectors.
// Run: GOCACHE=/tmp/go-cache GOTELEMETRY=off go run tooling/x509_vectors.go > seed/tests/fixtures/x509-vectors.json
package main

import (
	"crypto/ecdh"
	"crypto/ecdsa"
	"crypto/ed25519"
	"crypto/elliptic"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"crypto/x509/pkix"
	"encoding/asn1"
	"encoding/hex"
	"encoding/json"
	"encoding/pem"
	"fmt"
	"math/big"
	"net"
	"net/url"
	"os"
	"time"
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

func must[T any](v T, err error) T {
	if err != nil {
		panic(err)
	}
	return v
}

func mustOK(err error) {
	if err != nil {
		panic(err)
	}
}

func hx(b []byte) string {
	return hex.EncodeToString(b)
}

type PkixNameVector struct {
	Name               string   `json:"name"`
	Der                string   `json:"der"`
	String             string   `json:"string"`
	CommonName         string   `json:"common_name"`
	SerialNumber       string   `json:"serial_number"`
	Country            []string `json:"country"`
	Organization       []string `json:"organization"`
	OrganizationalUnit []string `json:"organizational_unit"`
	Locality           []string `json:"locality"`
	Province           []string `json:"province"`
	StreetAddress      []string `json:"street_address"`
	PostalCode         []string `json:"postal_code"`
}

type KeyCodecVector struct {
	Name            string `json:"name"`
	Kind            string `json:"kind"`
	PkixPublicDer   string `json:"pkix_public_der"`
	Pkcs8PrivateDer string `json:"pkcs8_private_der"`
	Sec1PrivateDer  string `json:"sec1_private_der,omitempty"`
	Pkcs1PublicDer  string `json:"pkcs1_public_der,omitempty"`
	Pkcs1PrivateDer string `json:"pkcs1_private_der,omitempty"`
	RawPublic       string `json:"raw_public,omitempty"`
	RawPrivate      string `json:"raw_private,omitempty"`
}

type CertVector struct {
	Name                string   `json:"name"`
	Der                 string   `json:"der"`
	Version             int      `json:"version"`
	Serial              string   `json:"serial"`
	SigAlg              string   `json:"sig_alg"`
	Issuer              string   `json:"issuer"`
	Subject             string   `json:"subject"`
	NotBefore           int64    `json:"not_before"`
	NotAfter            int64    `json:"not_after"`
	IsCA                bool     `json:"is_ca"`
	BasicConstraints    bool     `json:"basic_constraints_valid"`
	MaxPathLen          int      `json:"max_path_len"`
	MaxPathLenZero      bool     `json:"max_path_len_zero"`
	KeyUsage            uint32   `json:"key_usage"`
	ExtKeyUsages        []string `json:"ext_key_usages"`
	DnsNames            []string `json:"dns_names"`
	EmailAddresses      []string `json:"email_addresses"`
	IpAddresses         []string `json:"ip_addresses"`
	Uris                []string `json:"uris"`
	SubjectKeyId        string   `json:"subject_key_id"`
	AuthorityKeyId      string   `json:"authority_key_id"`
	PermittedDns        []string `json:"permitted_dns"`
	ExcludedDns         []string `json:"excluded_dns"`
	PermittedIpRanges   []string `json:"permitted_ip_ranges"`
	ExcludedIpRanges    []string `json:"excluded_ip_ranges"`
	RawTbs              string   `json:"raw_tbs"`
	Signature           string   `json:"signature"`
	PublicKeyKind       string   `json:"public_key_kind"`
}

type HostnameCase struct {
	Cert  string `json:"cert"`
	Host  string `json:"host"`
	Valid bool   `json:"valid"`
}

type VerifyCase struct {
	Name          string   `json:"name"`
	Leaf          string   `json:"leaf"`
	Intermediates []string `json:"intermediates"`
	Roots         []string `json:"roots"`
	DnsName       string   `json:"dns_name"`
	CurrentTime   int64    `json:"current_time"`
	KeyUsages     []string `json:"key_usages"`
	ExpectOk      bool     `json:"expect_ok"`
	ExpectReason  string   `json:"expect_reason"`
	ChainLen      int      `json:"chain_len"`
}

type CsrVector struct {
	Name           string   `json:"name"`
	Der            string   `json:"der"`
	Subject        string   `json:"subject"`
	SigAlg         string   `json:"sig_alg"`
	DnsNames       []string `json:"dns_names"`
	EmailAddresses []string `json:"email_addresses"`
	IpAddresses    []string `json:"ip_addresses"`
	Uris           []string `json:"uris"`
	ValidSig       bool     `json:"valid_sig"`
}

type RevokedEntryVector struct {
	Serial         string `json:"serial"`
	RevocationTime int64  `json:"revocation_time"`
	ExtCount       int    `json:"ext_count"`
}

type CrlVector struct {
	Name        string               `json:"name"`
	Der         string               `json:"der"`
	IssuerCert  string               `json:"issuer_cert"`
	Issuer      string               `json:"issuer"`
	ThisUpdate  int64                `json:"this_update"`
	NextUpdate  int64                `json:"next_update"`
	Revoked     []RevokedEntryVector `json:"revoked"`
	ValidSig    bool                 `json:"valid_sig"`
}

type Fixture struct {
	PkixNames     []PkixNameVector `json:"pkix_names"`
	Keys          []KeyCodecVector `json:"keys"`
	BadKeyDers    []string         `json:"bad_key_ders"`
	Certs         []CertVector     `json:"certs"`
	PemBundle     string           `json:"pem_bundle"`
	PemCertCount  int              `json:"pem_cert_count"`
	HostnameCases []HostnameCase   `json:"hostname_cases"`
	VerifyCases   []VerifyCase     `json:"verify_cases"`
	Csrs          []CsrVector      `json:"csrs"`
	Crls          []CrlVector      `json:"crls"`
}

func sigAlgName(a x509.SignatureAlgorithm) string {
	switch a {
	case x509.SHA1WithRSA:
		return "sha1_rsa"
	case x509.SHA256WithRSA:
		return "sha256_rsa"
	case x509.SHA384WithRSA:
		return "sha384_rsa"
	case x509.SHA512WithRSA:
		return "sha512_rsa"
	case x509.SHA256WithRSAPSS:
		return "sha256_rsapss"
	case x509.SHA384WithRSAPSS:
		return "sha384_rsapss"
	case x509.SHA512WithRSAPSS:
		return "sha512_rsapss"
	case x509.ECDSAWithSHA256:
		return "ecdsa_sha256"
	case x509.ECDSAWithSHA384:
		return "ecdsa_sha384"
	case x509.ECDSAWithSHA512:
		return "ecdsa_sha512"
	case x509.PureEd25519:
		return "ed25519"
	default:
		return fmt.Sprintf("unknown_%d", int(a))
	}
}

func ekuName(u x509.ExtKeyUsage) string {
	switch u {
	case x509.ExtKeyUsageAny:
		return "any"
	case x509.ExtKeyUsageServerAuth:
		return "server_auth"
	case x509.ExtKeyUsageClientAuth:
		return "client_auth"
	case x509.ExtKeyUsageCodeSigning:
		return "code_signing"
	case x509.ExtKeyUsageEmailProtection:
		return "email_protection"
	case x509.ExtKeyUsageIPSECEndSystem:
		return "ipsec_end_system"
	case x509.ExtKeyUsageIPSECTunnel:
		return "ipsec_tunnel"
	case x509.ExtKeyUsageIPSECUser:
		return "ipsec_user"
	case x509.ExtKeyUsageTimeStamping:
		return "time_stamping"
	case x509.ExtKeyUsageOCSPSigning:
		return "ocsp_signing"
	default:
		return "other"
	}
}

func pubKind(pub any) string {
	switch pub.(type) {
	case *rsa.PublicKey:
		return "rsa"
	case *ecdsa.PublicKey:
		return "p256"
	case ed25519.PublicKey:
		return "ed25519"
	case *ecdh.PublicKey:
		return "x25519"
	default:
		return "unknown"
	}
}

func makeCertVector(name string, der []byte) CertVector {
	c := must(x509.ParseCertificate(der))
	ekus := make([]string, 0, len(c.ExtKeyUsage))
	for _, u := range c.ExtKeyUsage {
		ekus = append(ekus, ekuName(u))
	}
	ips := make([]string, 0, len(c.IPAddresses))
	for _, ip := range c.IPAddresses {
		if ip4 := ip.To4(); ip4 != nil {
			ips = append(ips, ip4.String())
		} else {
			ips = append(ips, ip.String())
		}
	}
	uris := make([]string, 0, len(c.URIs))
	for _, u := range c.URIs {
		uris = append(uris, u.String())
	}
	permIPs := make([]string, 0, len(c.PermittedIPRanges))
	for _, n := range c.PermittedIPRanges {
		permIPs = append(permIPs, n.String())
	}
	exclIPs := make([]string, 0, len(c.ExcludedIPRanges))
	for _, n := range c.ExcludedIPRanges {
		exclIPs = append(exclIPs, n.String())
	}
	dns := c.DNSNames
	if dns == nil {
		dns = []string{}
	}
	emails := c.EmailAddresses
	if emails == nil {
		emails = []string{}
	}
	permDns := c.PermittedDNSDomains
	if permDns == nil {
		permDns = []string{}
	}
	exclDns := c.ExcludedDNSDomains
	if exclDns == nil {
		exclDns = []string{}
	}
	return CertVector{
		Name:              name,
		Der:               hx(der),
		Version:           c.Version,
		Serial:            c.SerialNumber.String(),
		SigAlg:            sigAlgName(c.SignatureAlgorithm),
		Issuer:            c.Issuer.String(),
		Subject:           c.Subject.String(),
		NotBefore:         c.NotBefore.Unix(),
		NotAfter:          c.NotAfter.Unix(),
		IsCA:              c.IsCA,
		BasicConstraints:  c.BasicConstraintsValid,
		MaxPathLen:        c.MaxPathLen,
		MaxPathLenZero:    c.MaxPathLenZero,
		KeyUsage:          uint32(c.KeyUsage),
		ExtKeyUsages:      ekus,
		DnsNames:          dns,
		EmailAddresses:    emails,
		IpAddresses:       ips,
		Uris:              uris,
		SubjectKeyId:      hx(c.SubjectKeyId),
		AuthorityKeyId:    hx(c.AuthorityKeyId),
		PermittedDns:      permDns,
		ExcludedDns:       exclDns,
		PermittedIpRanges: permIPs,
		ExcludedIpRanges:  exclIPs,
		RawTbs:            hx(c.RawTBSCertificate),
		Signature:         hx(c.Signature),
		PublicKeyKind:     pubKind(c.PublicKey),
	}
}

func mustCIDR(s string) *net.IPNet {
	_, ipnet, err := net.ParseCIDR(s)
	mustOK(err)
	return ipnet
}

func mustURL(s string) *url.URL {
	u, err := url.Parse(s)
	mustOK(err)
	return u
}

func nonNil(s []string) []string {
	if s == nil {
		return []string{}
	}
	return s
}

func makePkixNameVector(label string, n pkix.Name) PkixNameVector {
	rdns := n.ToRDNSequence()
	der := must(asn1.Marshal(rdns))
	var parsed pkix.RDNSequence
	_, err := asn1.Unmarshal(der, &parsed)
	mustOK(err)
	var out pkix.Name
	out.FillFromRDNSequence(&parsed)
	return PkixNameVector{
		Name:               label,
		Der:                hx(der),
		String:             out.String(),
		CommonName:         out.CommonName,
		SerialNumber:       out.SerialNumber,
		Country:            nonNil(out.Country),
		Organization:       nonNil(out.Organization),
		OrganizationalUnit: nonNil(out.OrganizationalUnit),
		Locality:           nonNil(out.Locality),
		Province:           nonNil(out.Province),
		StreetAddress:      nonNil(out.StreetAddress),
		PostalCode:         nonNil(out.PostalCode),
	}
}

func flipByte(b []byte, idx int) []byte {
	cp := make([]byte, len(b))
	copy(cp, b)
	cp[idx] ^= 0x01
	return cp
}

func main() {
	rng := newDetReader("cinch-x509-vectors-v1")

	// 1. PKIX Names
	pkixNames := []PkixNameVector{
		makePkixNameVector("full_dn", pkix.Name{
			Country:            []string{"US"},
			Province:           []string{"California"},
			Locality:           []string{"San Francisco"},
			StreetAddress:      []string{"100 Market St"},
			PostalCode:         []string{"94105"},
			Organization:       []string{"Cinch Security"},
			OrganizationalUnit: []string{"Platform"},
			CommonName:         "api.example.com",
			SerialNumber:       "SN-2026-001",
		}),
		makePkixNameVector("escaped_dn", pkix.Name{
			Country:      []string{"US"},
			Organization: []string{"Acme, Inc. + <Labs> \"#1\" ; \\test"},
			CommonName:   " #leading and trailing ",
		}),
		makePkixNameVector("multi_rdn", pkix.Name{
			Country:            []string{"US", "CA"},
			Organization:       []string{"Org One", "Org Two"},
			OrganizationalUnit: []string{"Eng", "Ops"},
			CommonName:         "multi.example.org",
		}),
		makePkixNameVector("extra_oid", pkix.Name{
			CommonName: "custom-oid.example.com",
			ExtraNames: []pkix.AttributeTypeAndValue{
				{
					Type:  asn1.ObjectIdentifier{1, 2, 3, 4, 5},
					Value: "custom-val",
				},
			},
		}),
		makePkixNameVector("empty", pkix.Name{}),
	}

	// 2. Keys (RSA-2048, ECDSA P-256, Ed25519, X25519)
	rsaRootKey := must(rsa.GenerateKey(newDetReader("rsa-root-2048"), 2048))
	rsaLeafKey := must(rsa.GenerateKey(newDetReader("rsa-leaf-2048"), 2048))
	p256InterKey := must(ecdsa.GenerateKey(elliptic.P256(), newDetReader("p256-inter")))
	p256LeafKey := must(ecdsa.GenerateKey(elliptic.P256(), newDetReader("p256-leaf")))
	edRootPub, edRootPriv := func() (ed25519.PublicKey, ed25519.PrivateKey) {
		pub, priv, err := ed25519.GenerateKey(newDetReader("ed25519-root"))
		mustOK(err)
		return pub, priv
	}()
	edLeafPub, edLeafPriv := func() (ed25519.PublicKey, ed25519.PrivateKey) {
		pub, priv, err := ed25519.GenerateKey(newDetReader("ed25519-leaf"))
		mustOK(err)
		return pub, priv
	}()
	x25519Priv := must(ecdh.X25519().GenerateKey(newDetReader("x25519-key")))
	x25519Pub := x25519Priv.PublicKey()

	p256Scalar := make([]byte, 32)
	p256InterKey.D.FillBytes(p256Scalar)
	p256Uncomp := elliptic.Marshal(elliptic.P256(), p256InterKey.X, p256InterKey.Y)

	keys := []KeyCodecVector{
		{
			Name:            "rsa2048",
			Kind:            "rsa",
			PkixPublicDer:   hx(must(x509.MarshalPKIXPublicKey(&rsaRootKey.PublicKey))),
			Pkcs8PrivateDer: hx(must(x509.MarshalPKCS8PrivateKey(rsaRootKey))),
			Pkcs1PublicDer:  hx(x509.MarshalPKCS1PublicKey(&rsaRootKey.PublicKey)),
			Pkcs1PrivateDer: hx(x509.MarshalPKCS1PrivateKey(rsaRootKey)),
		},
		{
			Name:            "p256",
			Kind:            "p256",
			PkixPublicDer:   hx(must(x509.MarshalPKIXPublicKey(&p256InterKey.PublicKey))),
			Pkcs8PrivateDer: hx(must(x509.MarshalPKCS8PrivateKey(p256InterKey))),
			Sec1PrivateDer:  hx(must(x509.MarshalECPrivateKey(p256InterKey))),
			RawPublic:       hx(p256Uncomp),
			RawPrivate:      hx(p256Scalar),
		},
		{
			Name:            "ed25519",
			Kind:            "ed25519",
			PkixPublicDer:   hx(must(x509.MarshalPKIXPublicKey(edRootPub))),
			Pkcs8PrivateDer: hx(must(x509.MarshalPKCS8PrivateKey(edRootPriv))),
			RawPublic:       hx(edRootPub),
			RawPrivate:      hx(edRootPriv.Seed()),
		},
		{
			Name:            "x25519",
			Kind:            "x25519",
			PkixPublicDer:   hx(must(x509.MarshalPKIXPublicKey(x25519Pub))),
			Pkcs8PrivateDer: hx(must(x509.MarshalPKCS8PrivateKey(x25519Priv))),
			RawPublic:       hx(x25519Pub.Bytes()),
			RawPrivate:      hx(x25519Priv.Bytes()),
		},
	}

	// Verify all keys round-trip in Go.
	for _, kv := range keys {
		must(x509.ParsePKIXPublicKey(must(hex.DecodeString(kv.PkixPublicDer))))
		must(x509.ParsePKCS8PrivateKey(must(hex.DecodeString(kv.Pkcs8PrivateDer))))
		if kv.Sec1PrivateDer != "" {
			must(x509.ParseECPrivateKey(must(hex.DecodeString(kv.Sec1PrivateDer))))
		}
	}

	badKeyDers := []string{
		"",
		"3000",
		"04020102",
		"3006020100040100",
	}

	// Timestamps
	t2020 := time.Date(2020, 1, 1, 0, 0, 0, 0, time.UTC)
	t2021 := time.Date(2021, 1, 1, 0, 0, 0, 0, time.UTC)
	t2024 := time.Date(2024, 1, 1, 0, 0, 0, 0, time.UTC)
	t2025 := time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC)
	t2026 := time.Date(2026, 6, 15, 12, 0, 0, 0, time.UTC)
	t2027 := time.Date(2027, 1, 1, 0, 0, 0, 0, time.UTC)
	t2030 := time.Date(2030, 1, 1, 0, 0, 0, 0, time.UTC)
	t2031 := time.Date(2031, 1, 1, 0, 0, 0, 0, time.UTC)
	t2034 := time.Date(2034, 1, 1, 0, 0, 0, 0, time.UTC)
	t2055 := time.Date(2055, 5, 5, 5, 5, 5, 0, time.UTC) // GeneralizedTime test!

	// 3. Certificates
	certDerMap := map[string][]byte{}
	certObjMap := map[string]*x509.Certificate{}
	var certVectors []CertVector
	addCert := func(name string, der []byte) *x509.Certificate {
		certDerMap[name] = der
		c := must(x509.ParseCertificate(der))
		certObjMap[name] = c
		certVectors = append(certVectors, makeCertVector(name, der))
		return c
	}

	// Root CA 1 (RSA-2048, SHA256WithRSA, GeneralizedTime NotAfter in 2055)
	root1Tmpl := &x509.Certificate{
		SerialNumber: big.NewInt(1),
		Subject: pkix.Name{
			Country:      []string{"US"},
			Organization: []string{"Cinch Root CA"},
			CommonName:   "Cinch RSA Root CA",
		},
		NotBefore:             t2024,
		NotAfter:              t2055,
		KeyUsage:              x509.KeyUsageCertSign | x509.KeyUsageCRLSign,
		BasicConstraintsValid: true,
		IsCA:                  true,
		MaxPathLen:            1,
		SubjectKeyId:          []byte{0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08},
		SignatureAlgorithm:    x509.SHA256WithRSA,
	}
	root1 := addCert("root_rsa", must(x509.CreateCertificate(rng, root1Tmpl, root1Tmpl, &rsaRootKey.PublicKey, rsaRootKey)))

	// Intermediate CA 1 (P-256, signed by Root CA 1, MaxPathLen=0, NameConstraints)
	inter1Tmpl := &x509.Certificate{
		SerialNumber: big.NewInt(100),
		Subject: pkix.Name{
			Country:      []string{"US"},
			Organization: []string{"Cinch Intermediate CA"},
			CommonName:   "Cinch P256 Intermediate CA",
		},
		NotBefore:               t2024,
		NotAfter:                t2034,
		KeyUsage:                x509.KeyUsageCertSign | x509.KeyUsageCRLSign,
		BasicConstraintsValid:   true,
		IsCA:                    true,
		MaxPathLen:              0,
		MaxPathLenZero:          true,
		SubjectKeyId:            []byte{0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88},
		PermittedDNSDomainsCritical: true,
		PermittedDNSDomains:     []string{"example.com", ".allowed.org"},
		ExcludedDNSDomains:      []string{"bad.example.com"},
		PermittedIPRanges:       []*net.IPNet{mustCIDR("192.0.2.0/24"), mustCIDR("2001:db8::/32")},
		ExcludedIPRanges:        []*net.IPNet{mustCIDR("192.0.2.128/25")},
		SignatureAlgorithm:      x509.SHA256WithRSA,
	}
	inter1 := addCert("inter_p256", must(x509.CreateCertificate(rng, inter1Tmpl, root1, &p256InterKey.PublicKey, rsaRootKey)))

	// Leaf 1 (P-256, signed by Intermediate CA 1 with ECDSAWithSHA256)
	leaf1Tmpl := &x509.Certificate{
		SerialNumber: big.NewInt(1001),
		Subject: pkix.Name{
			Organization: []string{"Example Corp"},
			CommonName:   "www.example.com",
		},
		NotBefore:          t2025,
		NotAfter:           t2027,
		KeyUsage:           x509.KeyUsageDigitalSignature,
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth, x509.ExtKeyUsageClientAuth},
		DNSNames:           []string{"www.example.com", "*.sub.example.com", "api.allowed.org"},
		EmailAddresses:     []string{"ops@example.com"},
		IPAddresses:        []net.IP{net.ParseIP("192.0.2.10").To4(), net.ParseIP("2001:db8::42")},
		URIs:               []*url.URL{mustURL("https://www.example.com/service")},
		SignatureAlgorithm: x509.ECDSAWithSHA256,
	}
	leaf1 := addCert("leaf_p256", must(x509.CreateCertificate(rng, leaf1Tmpl, inter1, &p256LeafKey.PublicKey, p256InterKey)))

	// Root CA 2 (Ed25519)
	root2Tmpl := &x509.Certificate{
		SerialNumber: big.NewInt(2),
		Subject: pkix.Name{
			Organization: []string{"Cinch Ed25519 Root"},
			CommonName:   "Cinch Ed25519 Root CA",
		},
		NotBefore:             t2024,
		NotAfter:              t2034,
		KeyUsage:              x509.KeyUsageCertSign | x509.KeyUsageCRLSign,
		BasicConstraintsValid: true,
		IsCA:                  true,
		SubjectKeyId:          []byte{0xaa, 0xbb, 0xcc, 0xdd},
		SignatureAlgorithm:    x509.PureEd25519,
	}
	root2 := addCert("root_ed25519", must(x509.CreateCertificate(rng, root2Tmpl, root2Tmpl, edRootPub, edRootPriv)))

	// Leaf 2 (Ed25519, signed by Root CA 2)
	leaf2Tmpl := &x509.Certificate{
		SerialNumber: big.NewInt(2001),
		Subject: pkix.Name{
			CommonName: "ed.example.net",
		},
		NotBefore:          t2025,
		NotAfter:           t2027,
		KeyUsage:           x509.KeyUsageDigitalSignature,
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		DNSNames:           []string{"ed.example.net", "*.wild.example.net"},
		IPAddresses:        []net.IP{net.ParseIP("10.0.0.1").To4()},
		SignatureAlgorithm: x509.PureEd25519,
	}
	_ = addCert("leaf_ed25519", must(x509.CreateCertificate(rng, leaf2Tmpl, root2, edLeafPub, edRootPriv)))

	// Leaf 3 (RSA-2048, signed by Root CA 1 with SHA256WithRSAPSS)
	leafPssTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(3001),
		Subject:            pkix.Name{CommonName: "pss.example.com"},
		NotBefore:          t2025,
		NotAfter:           t2027,
		KeyUsage:           x509.KeyUsageDigitalSignature,
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		DNSNames:           []string{"pss.example.com"},
		SignatureAlgorithm: x509.SHA256WithRSAPSS,
	}
	_ = addCert("leaf_rsapss_sha256", must(x509.CreateCertificate(rng, leafPssTmpl, root1, &rsaLeafKey.PublicKey, rsaRootKey)))

	// Leaf 4 (RSA SHA-384), Leaf 5 (RSA SHA-512), Leaf 6 (ECDSA SHA-384), Leaf 7 (ECDSA SHA-512)
	for _, spec := range []struct {
		name   string
		serial int64
		alg    x509.SignatureAlgorithm
		parent *x509.Certificate
		signer any
	}{
		{"leaf_rsa_sha384", 3002, x509.SHA384WithRSA, root1, rsaRootKey},
		{"leaf_rsa_sha512", 3003, x509.SHA512WithRSA, root1, rsaRootKey},
		{"leaf_rsapss_sha384", 3004, x509.SHA384WithRSAPSS, root1, rsaRootKey},
		{"leaf_ecdsa_sha384", 3005, x509.ECDSAWithSHA384, inter1, p256InterKey},
		{"leaf_ecdsa_sha512", 3006, x509.ECDSAWithSHA512, inter1, p256InterKey},
	} {
		tmpl := &x509.Certificate{
			SerialNumber:       big.NewInt(spec.serial),
			Subject:            pkix.Name{CommonName: spec.name + ".example.com"},
			NotBefore:          t2025,
			NotAfter:           t2027,
			KeyUsage:           x509.KeyUsageDigitalSignature,
			ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
			DNSNames:           []string{spec.name + ".example.com"},
			SignatureAlgorithm: spec.alg,
		}
		_ = addCert(spec.name, must(x509.CreateCertificate(rng, tmpl, spec.parent, &p256LeafKey.PublicKey, spec.signer)))
	}

	// CN-only leaf (no SAN -> RFC 6125 rejects hostname match)
	cnOnlyTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(3010),
		Subject:            pkix.Name{CommonName: "cnonly.example.net"},
		NotBefore:          t2025,
		NotAfter:           t2027,
		KeyUsage:           x509.KeyUsageDigitalSignature,
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		SignatureAlgorithm: x509.PureEd25519,
	}
	_ = addCert("leaf_cn_only", must(x509.CreateCertificate(rng, cnOnlyTmpl, root2, edLeafPub, edRootPriv)))

	// Negative certs:
	// Expired leaf
	expiredTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(4001),
		Subject:            pkix.Name{CommonName: "expired.example.net"},
		NotBefore:          t2020,
		NotAfter:           t2021,
		DNSNames:           []string{"expired.example.net"},
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		SignatureAlgorithm: x509.PureEd25519,
	}
	_ = addCert("leaf_expired", must(x509.CreateCertificate(rng, expiredTmpl, root2, edLeafPub, edRootPriv)))

	// Not-yet-valid leaf
	futureTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(4002),
		Subject:            pkix.Name{CommonName: "future.example.net"},
		NotBefore:          t2030,
		NotAfter:           t2031,
		DNSNames:           []string{"future.example.net"},
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		SignatureAlgorithm: x509.PureEd25519,
	}
	_ = addCert("leaf_future", must(x509.CreateCertificate(rng, futureTmpl, root2, edLeafPub, edRootPriv)))

	// Tampered leaf (flip last byte of signature of leaf_p256)
	tamperedDer := flipByte(certDerMap["leaf_p256"], len(certDerMap["leaf_p256"])-1)
	_ = addCert("leaf_tampered_sig", tamperedDer)

	// Non-CA intermediate signing a leaf
	nonCaInterTmpl := &x509.Certificate{
		SerialNumber:          big.NewInt(4010),
		Subject:               pkix.Name{CommonName: "Not A CA"},
		NotBefore:             t2024,
		NotAfter:              t2034,
		BasicConstraintsValid: true,
		IsCA:                  false,
		SignatureAlgorithm:    x509.PureEd25519,
	}
	nonCaInter := addCert("inter_non_ca", must(x509.CreateCertificate(rng, nonCaInterTmpl, root2, edLeafPub, edRootPriv)))
	leafByNonCaTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(4011),
		Subject:            pkix.Name{CommonName: "badca.example.net"},
		NotBefore:          t2025,
		NotAfter:           t2027,
		DNSNames:           []string{"badca.example.net"},
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		SignatureAlgorithm: x509.PureEd25519,
	}
	_ = addCert("leaf_by_non_ca", must(x509.CreateCertificate(rng, leafByNonCaTmpl, nonCaInter, edLeafPub, edLeafPriv)))

	// MaxPathLen violation: inter1 has MaxPathLen=0, signs inter2 (CA), which signs leaf
	inter2Key := must(ecdsa.GenerateKey(elliptic.P256(), newDetReader("p256-inter2")))
	inter2Tmpl := &x509.Certificate{
		SerialNumber:          big.NewInt(4020),
		Subject:               pkix.Name{CommonName: "Sub-Intermediate CA"},
		NotBefore:             t2024,
		NotAfter:              t2034,
		KeyUsage:              x509.KeyUsageCertSign,
		BasicConstraintsValid: true,
		IsCA:                  true,
		SignatureAlgorithm:    x509.ECDSAWithSHA256,
	}
	inter2 := addCert("inter_pathlen_sub", must(x509.CreateCertificate(rng, inter2Tmpl, inter1, &inter2Key.PublicKey, p256InterKey)))
	leafPathlenTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(4021),
		Subject:            pkix.Name{CommonName: "deep.example.com"},
		NotBefore:          t2025,
		NotAfter:           t2027,
		DNSNames:           []string{"deep.example.com"},
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		SignatureAlgorithm: x509.ECDSAWithSHA256,
	}
	_ = addCert("leaf_pathlen_viol", must(x509.CreateCertificate(rng, leafPathlenTmpl, inter2, &p256LeafKey.PublicKey, inter2Key)))

	// NameConstraints violations (signed by inter1):
	ncDnsExclTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(4030),
		Subject:            pkix.Name{CommonName: "bad.example.com"},
		NotBefore:          t2025,
		NotAfter:           t2027,
		DNSNames:           []string{"bad.example.com"},
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		SignatureAlgorithm: x509.ECDSAWithSHA256,
	}
	_ = addCert("leaf_nc_dns_excl", must(x509.CreateCertificate(rng, ncDnsExclTmpl, inter1, &p256LeafKey.PublicKey, p256InterKey)))

	ncDnsNotPermTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(4031),
		Subject:            pkix.Name{CommonName: "other.net"},
		NotBefore:          t2025,
		NotAfter:           t2027,
		DNSNames:           []string{"other.net"},
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		SignatureAlgorithm: x509.ECDSAWithSHA256,
	}
	_ = addCert("leaf_nc_dns_not_perm", must(x509.CreateCertificate(rng, ncDnsNotPermTmpl, inter1, &p256LeafKey.PublicKey, p256InterKey)))

	ncIpExclTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(4032),
		Subject:            pkix.Name{CommonName: "ip-excl.example.com"},
		NotBefore:          t2025,
		NotAfter:           t2027,
		DNSNames:           []string{"ip-excl.example.com"},
		IPAddresses:        []net.IP{net.ParseIP("192.0.2.200").To4()},
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		SignatureAlgorithm: x509.ECDSAWithSHA256,
	}
	_ = addCert("leaf_nc_ip_excl", must(x509.CreateCertificate(rng, ncIpExclTmpl, inter1, &p256LeafKey.PublicKey, p256InterKey)))

	ncIpNotPermTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(4033),
		Subject:            pkix.Name{CommonName: "ip-not-perm.example.com"},
		NotBefore:          t2025,
		NotAfter:           t2027,
		DNSNames:           []string{"ip-not-perm.example.com"},
		IPAddresses:        []net.IP{net.ParseIP("198.51.100.1").To4()},
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		SignatureAlgorithm: x509.ECDSAWithSHA256,
	}
	_ = addCert("leaf_nc_ip_not_perm", must(x509.CreateCertificate(rng, ncIpNotPermTmpl, inter1, &p256LeafKey.PublicKey, p256InterKey)))

	// Wrong EKU leaf (EmailProtection only)
	wrongEkuTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(4040),
		Subject:            pkix.Name{CommonName: "mail.example.net"},
		NotBefore:          t2025,
		NotAfter:           t2027,
		DNSNames:           []string{"mail.example.net"},
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageEmailProtection},
		SignatureAlgorithm: x509.PureEd25519,
	}
	_ = addCert("leaf_email_eku", must(x509.CreateCertificate(rng, wrongEkuTmpl, root2, edLeafPub, edRootPriv)))

	// Unhandled critical extension leaf
	critExtTmpl := &x509.Certificate{
		SerialNumber:       big.NewInt(4050),
		Subject:            pkix.Name{CommonName: "crit.example.net"},
		NotBefore:          t2025,
		NotAfter:           t2027,
		DNSNames:           []string{"crit.example.net"},
		ExtKeyUsage:        []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		ExtraExtensions: []pkix.Extension{
			{
				Id:       asn1.ObjectIdentifier{1, 2, 3, 4, 5, 6, 7},
				Critical: true,
				Value:    []byte{0x05, 0x00},
			},
		},
		SignatureAlgorithm: x509.PureEd25519,
	}
	_ = addCert("leaf_crit_ext", must(x509.CreateCertificate(rng, critExtTmpl, root2, edLeafPub, edRootPriv)))

	// PEM bundle (3 certs + 1 unrelated PEM block)
	pemBundle := string(pem.EncodeToMemory(&pem.Block{Type: "CERTIFICATE", Bytes: certDerMap["leaf_p256"]})) +
		string(pem.EncodeToMemory(&pem.Block{Type: "NOTE", Bytes: []byte("ignored block")})) +
		string(pem.EncodeToMemory(&pem.Block{Type: "CERTIFICATE", Bytes: certDerMap["inter_p256"]})) +
		string(pem.EncodeToMemory(&pem.Block{Type: "CERTIFICATE", Bytes: certDerMap["root_rsa"]}))

	// 4. Hostname Verification Cases (cross-checked directly against Go!)
	hostTests := []struct {
		cert string
		host string
	}{
		{"leaf_p256", "www.example.com"},
		{"leaf_p256", "WWW.EXAMPLE.COM"},
		{"leaf_p256", "www.example.com."},
		{"leaf_p256", "a.sub.example.com"},
		{"leaf_p256", "A.SUB.EXAMPLE.COM"},
		{"leaf_p256", "a.b.sub.example.com"},
		{"leaf_p256", "sub.example.com"},
		{"leaf_p256", "api.allowed.org"},
		{"leaf_p256", "other.example.com"},
		{"leaf_p256", "192.0.2.10"},
		{"leaf_p256", "192.0.2.11"},
		{"leaf_p256", "2001:db8::42"},
		{"leaf_p256", "[2001:db8::42]"},
		{"leaf_p256", "2001:db8::43"},
		{"leaf_ed25519", "ed.example.net"},
		{"leaf_ed25519", "foo.wild.example.net"},
		{"leaf_ed25519", "10.0.0.1"},
		{"leaf_cn_only", "cnonly.example.net"},
	}
	var hostnameCases []HostnameCase
	for _, ht := range hostTests {
		c := certObjMap[ht.cert]
		ok := c.VerifyHostname(ht.host) == nil
		hostnameCases = append(hostnameCases, HostnameCase{
			Cert:  ht.cert,
			Host:  ht.host,
			Valid: ok,
		})
	}

	// 5. Chain Verification Cases (cross-checked directly against Go!)
	verifySpecs := []VerifyCase{
		{
			Name:          "valid_3cert_rsa_p256_chain",
			Leaf:          "leaf_p256",
			Intermediates: []string{"inter_p256"},
			Roots:         []string{"root_rsa"},
			DnsName:       "www.example.com",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      true,
			ChainLen:      3,
		},
		{
			Name:          "valid_3cert_wildcard_host",
			Leaf:          "leaf_p256",
			Intermediates: []string{"inter_p256"},
			Roots:         []string{"root_rsa"},
			DnsName:       "x.sub.example.com",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"client_auth"},
			ExpectOk:      true,
			ChainLen:      3,
		},
		{
			Name:          "valid_ed25519_2cert",
			Leaf:          "leaf_ed25519",
			Intermediates: []string{},
			Roots:         []string{"root_ed25519"},
			DnsName:       "ed.example.net",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{},
			ExpectOk:      true,
			ChainLen:      2,
		},
		{
			Name:          "valid_rsapss_sha256",
			Leaf:          "leaf_rsapss_sha256",
			Intermediates: []string{},
			Roots:         []string{"root_rsa"},
			DnsName:       "pss.example.com",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      true,
			ChainLen:      2,
		},
		{
			Name:          "valid_email_eku_when_requested",
			Leaf:          "leaf_email_eku",
			Intermediates: []string{},
			Roots:         []string{"root_ed25519"},
			DnsName:       "mail.example.net",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"email_protection"},
			ExpectOk:      true,
			ChainLen:      2,
		},
		{
			Name:          "fail_wrong_hostname",
			Leaf:          "leaf_p256",
			Intermediates: []string{"inter_p256"},
			Roots:         []string{"root_rsa"},
			DnsName:       "wrong.example.com",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "hostname",
		},
		{
			Name:          "fail_expired",
			Leaf:          "leaf_expired",
			Intermediates: []string{},
			Roots:         []string{"root_ed25519"},
			DnsName:       "expired.example.net",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "expired",
		},
		{
			Name:          "fail_not_yet_valid",
			Leaf:          "leaf_future",
			Intermediates: []string{},
			Roots:         []string{"root_ed25519"},
			DnsName:       "future.example.net",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "expired",
		},
		{
			Name:          "fail_tampered_sig",
			Leaf:          "leaf_tampered_sig",
			Intermediates: []string{"inter_p256"},
			Roots:         []string{"root_rsa"},
			DnsName:       "www.example.com",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "signature",
		},
		{
			Name:          "fail_missing_intermediate",
			Leaf:          "leaf_p256",
			Intermediates: []string{},
			Roots:         []string{"root_rsa"},
			DnsName:       "www.example.com",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "unknown_authority",
		},
		{
			Name:          "fail_untrusted_root",
			Leaf:          "leaf_ed25519",
			Intermediates: []string{},
			Roots:         []string{"root_rsa"},
			DnsName:       "ed.example.net",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "unknown_authority",
		},
		{
			Name:          "fail_non_ca_issuer",
			Leaf:          "leaf_by_non_ca",
			Intermediates: []string{"inter_non_ca"},
			Roots:         []string{"root_ed25519"},
			DnsName:       "badca.example.net",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "not_ca",
		},
		{
			Name:          "fail_pathlen_violation",
			Leaf:          "leaf_pathlen_viol",
			Intermediates: []string{"inter_pathlen_sub", "inter_p256"},
			Roots:         []string{"root_rsa"},
			DnsName:       "deep.example.com",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "pathlen",
		},
		{
			Name:          "fail_nc_dns_excluded",
			Leaf:          "leaf_nc_dns_excl",
			Intermediates: []string{"inter_p256"},
			Roots:         []string{"root_rsa"},
			DnsName:       "bad.example.com",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "name_constraint",
		},
		{
			Name:          "fail_nc_dns_not_permitted",
			Leaf:          "leaf_nc_dns_not_perm",
			Intermediates: []string{"inter_p256"},
			Roots:         []string{"root_rsa"},
			DnsName:       "other.net",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "name_constraint",
		},
		{
			Name:          "fail_nc_ip_excluded",
			Leaf:          "leaf_nc_ip_excl",
			Intermediates: []string{"inter_p256"},
			Roots:         []string{"root_rsa"},
			DnsName:       "ip-excl.example.com",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "name_constraint",
		},
		{
			Name:          "fail_nc_ip_not_permitted",
			Leaf:          "leaf_nc_ip_not_perm",
			Intermediates: []string{"inter_p256"},
			Roots:         []string{"root_rsa"},
			DnsName:       "ip-not-perm.example.com",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "name_constraint",
		},
		{
			Name:          "fail_incompatible_eku",
			Leaf:          "leaf_email_eku",
			Intermediates: []string{},
			Roots:         []string{"root_ed25519"},
			DnsName:       "mail.example.net",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "eku",
		},
		{
			Name:          "fail_unhandled_critical_ext",
			Leaf:          "leaf_crit_ext",
			Intermediates: []string{},
			Roots:         []string{"root_ed25519"},
			DnsName:       "crit.example.net",
			CurrentTime:   t2026.Unix(),
			KeyUsages:     []string{"server_auth"},
			ExpectOk:      false,
			ExpectReason:  "critical_ext",
		},
	}

	// Self-verify every VerifyCase against Go's x509.Certificate.Verify!
	ekuFromName := func(s string) x509.ExtKeyUsage {
		switch s {
		case "server_auth":
			return x509.ExtKeyUsageServerAuth
		case "client_auth":
			return x509.ExtKeyUsageClientAuth
		case "email_protection":
			return x509.ExtKeyUsageEmailProtection
		case "any":
			return x509.ExtKeyUsageAny
		default:
			panic("unknown eku " + s)
		}
	}
	for _, vc := range verifySpecs {
		roots := x509.NewCertPool()
		for _, r := range vc.Roots {
			roots.AddCert(certObjMap[r])
		}
		inters := x509.NewCertPool()
		for _, i := range vc.Intermediates {
			inters.AddCert(certObjMap[i])
		}
		var usages []x509.ExtKeyUsage
		for _, u := range vc.KeyUsages {
			usages = append(usages, ekuFromName(u))
		}
		opts := x509.VerifyOptions{
			DNSName:       vc.DnsName,
			Intermediates: inters,
			Roots:         roots,
			CurrentTime:   time.Unix(vc.CurrentTime, 0).UTC(),
			KeyUsages:     usages,
		}
		chains, err := certObjMap[vc.Leaf].Verify(opts)
		if vc.ExpectOk && err != nil {
			panic(fmt.Sprintf("Go rejected expected-ok verify case %s: %v", vc.Name, err))
		}
		if !vc.ExpectOk && err == nil {
			panic(fmt.Sprintf("Go accepted expected-fail verify case %s", vc.Name))
		}
		if vc.ExpectOk && len(chains[0]) != vc.ChainLen {
			panic(fmt.Sprintf("Go chain len mismatch for %s: got %d want %d", vc.Name, len(chains[0]), vc.ChainLen))
		}
	}

	// Verify leaf1 CheckSignatureFrom(inter1) and inter1 CheckSignatureFrom(root1)
	mustOK(leaf1.CheckSignatureFrom(inter1))
	mustOK(inter1.CheckSignatureFrom(root1))

	// 6. CSRs
	makeCsr := func(name string, tmpl *x509.CertificateRequest, priv any) CsrVector {
		der := must(x509.CreateCertificateRequest(rng, tmpl, priv))
		csr := must(x509.ParseCertificateRequest(der))
		mustOK(csr.CheckSignature())
		ips := make([]string, 0, len(csr.IPAddresses))
		for _, ip := range csr.IPAddresses {
			if ip4 := ip.To4(); ip4 != nil {
				ips = append(ips, ip4.String())
			} else {
				ips = append(ips, ip.String())
			}
		}
		uris := make([]string, 0, len(csr.URIs))
		for _, u := range csr.URIs {
			uris = append(uris, u.String())
		}
		return CsrVector{
			Name:           name,
			Der:            hx(der),
			Subject:        csr.Subject.String(),
			SigAlg:         sigAlgName(csr.SignatureAlgorithm),
			DnsNames:       nonNil(csr.DNSNames),
			EmailAddresses: nonNil(csr.EmailAddresses),
			IpAddresses:    ips,
			Uris:           uris,
			ValidSig:       true,
		}
	}

	csrRsa := makeCsr("csr_rsa", &x509.CertificateRequest{
		Subject: pkix.Name{
			Country:      []string{"US"},
			Organization: []string{"Cinch"},
			CommonName:   "csr-rsa.example.com",
		},
		DNSNames:           []string{"csr-rsa.example.com", "alt.example.com"},
		EmailAddresses:     []string{"admin@example.com"},
		IPAddresses:        []net.IP{net.ParseIP("192.0.2.55").To4()},
		URIs:               []*url.URL{mustURL("https://csr-rsa.example.com/id")},
		SignatureAlgorithm: x509.SHA256WithRSA,
	}, rsaLeafKey)

	csrP256 := makeCsr("csr_p256", &x509.CertificateRequest{
		Subject: pkix.Name{
			Organization: []string{"Cinch EC"},
			CommonName:   "csr-p256.example.com",
		},
		DNSNames:           []string{"csr-p256.example.com"},
		SignatureAlgorithm: x509.ECDSAWithSHA256,
	}, p256LeafKey)

	csrEd := makeCsr("csr_ed25519", &x509.CertificateRequest{
		Subject: pkix.Name{
			CommonName: "csr-ed.example.net",
		},
		DNSNames:           []string{"csr-ed.example.net"},
		IPAddresses:        []net.IP{net.ParseIP("2001:db8::99")},
		SignatureAlgorithm: x509.PureEd25519,
	}, edLeafPriv)

	tamperedCsrDer := flipByte(must(hex.DecodeString(csrEd.Der)), len(must(hex.DecodeString(csrEd.Der)))-1)
	tamperedCsr := csrEd
	tamperedCsr.Name = "csr_ed25519_tampered"
	tamperedCsr.Der = hx(tamperedCsrDer)
	tamperedCsr.ValidSig = false

	csrs := []CsrVector{csrRsa, csrP256, csrEd, tamperedCsr}

	// 7. CRLs
	crl1Der := must(x509.CreateRevocationList(rng, &x509.RevocationList{
		Number:     big.NewInt(7),
		ThisUpdate: t2025,
		NextUpdate: t2027,
		RevokedCertificateEntries: []x509.RevocationListEntry{
			{
				SerialNumber:   big.NewInt(1001),
				RevocationTime: t2025.Add(24 * time.Hour),
				ReasonCode:     1, // keyCompromise
			},
			{
				SerialNumber:   big.NewInt(1002),
				RevocationTime: t2025.Add(48 * time.Hour),
			},
		},
		SignatureAlgorithm: x509.SHA256WithRSA,
	}, root1, rsaRootKey))
	crl1Parsed := must(x509.ParseRevocationList(crl1Der))
	mustOK(crl1Parsed.CheckSignatureFrom(root1))

	crl2Der := must(x509.CreateRevocationList(rng, &x509.RevocationList{
		Number:             big.NewInt(1),
		ThisUpdate:         t2025,
		NextUpdate:         t2027,
		SignatureAlgorithm: x509.PureEd25519,
	}, root2, edRootPriv))
	crl2Parsed := must(x509.ParseRevocationList(crl2Der))
	mustOK(crl2Parsed.CheckSignatureFrom(root2))

	crlTamperedDer := flipByte(crl2Der, len(crl2Der)-1)

	makeCrlVec := func(name, issuerCert string, der []byte, validSig bool) CrlVector {
		c := must(x509.ParseRevocationList(der))
		revs := make([]RevokedEntryVector, 0, len(c.RevokedCertificateEntries))
		for _, r := range c.RevokedCertificateEntries {
			revs = append(revs, RevokedEntryVector{
				Serial:         r.SerialNumber.String(),
				RevocationTime: r.RevocationTime.Unix(),
				ExtCount:       len(r.Extensions),
			})
		}
		return CrlVector{
			Name:       name,
			Der:        hx(der),
			IssuerCert: issuerCert,
			Issuer:     c.Issuer.String(),
			ThisUpdate: c.ThisUpdate.Unix(),
			NextUpdate: c.NextUpdate.Unix(),
			Revoked:    revs,
			ValidSig:   validSig,
		}
	}

	crls := []CrlVector{
		makeCrlVec("crl_rsa_revoked", "root_rsa", crl1Der, true),
		makeCrlVec("crl_ed25519_empty", "root_ed25519", crl2Der, true),
		makeCrlVec("crl_ed25519_tampered", "root_ed25519", crlTamperedDer, false),
	}

	fixture := Fixture{
		PkixNames:     pkixNames,
		Keys:          keys,
		BadKeyDers:    badKeyDers,
		Certs:         certVectors,
		PemBundle:     pemBundle,
		PemCertCount:  3,
		HostnameCases: hostnameCases,
		VerifyCases:   verifySpecs,
		Csrs:          csrs,
		Crls:          crls,
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	mustOK(enc.Encode(fixture))
}
