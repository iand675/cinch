package main

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"net/netip"
	"os"
)

type AddrRow struct {
	Input           string  `json:"input"`
	Valid           bool    `json:"valid"`
	String          string  `json:"string,omitempty"`
	Expanded        string  `json:"expanded,omitempty"`
	Is4             bool    `json:"is4,omitempty"`
	Is6             bool    `json:"is6,omitempty"`
	Is4In6          bool    `json:"is4in6,omitempty"`
	BitLen          int     `json:"bit_len,omitempty"`
	Zone            string  `json:"zone,omitempty"`
	Unmap           string  `json:"unmap,omitempty"`
	As4Hex          *string `json:"as4_hex,omitempty"`
	As16Hex         string  `json:"as16_hex,omitempty"`
	AsSliceHex      string  `json:"as_slice_hex,omitempty"`
	MarshalBinHex   string  `json:"marshal_bin_hex,omitempty"`
	Next            *string `json:"next,omitempty"`
	Prev            *string `json:"prev,omitempty"`
	IsUnspecified   bool    `json:"is_unspecified,omitempty"`
	IsLoopback      bool    `json:"is_loopback,omitempty"`
	IsMulticast     bool    `json:"is_multicast,omitempty"`
	IsIfLocalMC     bool    `json:"is_if_local_mc,omitempty"`
	IsLinkLocalMC   bool    `json:"is_link_local_mc,omitempty"`
	IsLinkLocalUC   bool    `json:"is_link_local_uc,omitempty"`
	IsGlobalUnicast bool    `json:"is_global_unicast,omitempty"`
	IsPrivate       bool    `json:"is_private,omitempty"`
	WithZoneEth0    string  `json:"with_zone_eth0,omitempty"`
	WithZoneEmpty   string  `json:"with_zone_empty,omitempty"`
}

type CompareRow struct {
	A   string `json:"a"`
	B   string `json:"b"`
	Cmp int    `json:"cmp"`
}

type AddrPortRow struct {
	Input         string `json:"input"`
	Valid         bool   `json:"valid"`
	String        string `json:"string,omitempty"`
	Addr          string `json:"addr,omitempty"`
	Port          uint16 `json:"port,omitempty"`
	MarshalBinHex string `json:"marshal_bin_hex,omitempty"`
}

type PrefixRow struct {
	Input         string `json:"input"`
	Valid         bool   `json:"valid"`
	String        string `json:"string,omitempty"`
	Addr          string `json:"addr,omitempty"`
	Bits          int    `json:"bits,omitempty"`
	IsSingleIP    bool   `json:"is_single_ip,omitempty"`
	Masked        string `json:"masked,omitempty"`
	MarshalBinHex string `json:"marshal_bin_hex,omitempty"`
}

type AddrPrefixRow struct {
	Addr   string `json:"addr"`
	Bits   int    `json:"bits"`
	Valid  bool   `json:"valid"`
	Result string `json:"result,omitempty"`
}

type ContainsRow struct {
	Prefix   string `json:"prefix"`
	Addr     string `json:"addr"`
	Contains bool   `json:"contains"`
}

type OverlapsRow struct {
	A        string `json:"a"`
	B        string `json:"b"`
	Overlaps bool   `json:"overlaps"`
}

type Vectors struct {
	Addrs           []AddrRow       `json:"addrs"`
	CompareAddr     []CompareRow    `json:"compare_addr"`
	AddrPorts       []AddrPortRow   `json:"addr_ports"`
	CompareAddrPort []CompareRow    `json:"compare_addr_port"`
	Prefixes        []PrefixRow     `json:"prefixes"`
	AddrPrefixes    []AddrPrefixRow `json:"addr_prefixes"`
	Contains        []ContainsRow   `json:"contains"`
	Overlaps        []OverlapsRow   `json:"overlaps"`
}

func strPtr(s string) *string { return &s }

func main() {
	addrInputs := []string{
		// Valid IPv4
		"0.0.0.0",
		"1.2.3.4",
		"10.0.0.1",
		"127.0.0.1",
		"127.255.255.254",
		"169.254.0.1",
		"169.254.255.255",
		"172.15.255.255",
		"172.16.0.0",
		"172.31.255.255",
		"172.32.0.0",
		"192.168.0.1",
		"192.168.255.255",
		"224.0.0.0",
		"224.0.0.251",
		"224.0.1.0",
		"239.255.255.255",
		"240.0.0.1",
		"255.255.255.255",

		// Valid IPv6 & 4-in-6 & zones & RFC 5952 compression edge cases
		"::",
		"::1",
		"::%eth0",
		"::1%lo0",
		"::ffff:0.0.0.0",
		"::ffff:1.2.3.4",
		"::ffff:10.0.0.1",
		"::ffff:127.0.0.1",
		"::ffff:169.254.1.2",
		"::ffff:224.0.0.1",
		"::ffff:255.255.255.255",
		"::ffff:1.2.3.4%eth0",
		"0:0:0:0:0:ffff:0102:0304",
		"0:0:0:0:0:ffff:c000:0280",
		"::1.2.3.4",
		"1:2:3:4:5:6:1.2.3.4",
		"1::1.2.3.4",
		"2001:db8::1",
		"2001:0db8:0000:0000:0000:0000:0000:0001",
		"2001:db8:0:0:1:0:0:1",
		"2001:db8:0:1:0:0:1:1",
		"2001:db8:0:1:1:1:1:1",
		"2001:db8:0:0:1:0:0:0",
		"0:0:0:1:0:0:0:1",
		"0:0:1:0:0:0:0:0",
		"ABCD:ef01::",
		"0:0:0:0:0:0:0:0",
		"0::0",
		"fe80::1",
		"fe80::1%eth0",
		"fe80::1%1",
		"fe80::1%a%b",
		"fe7f:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
		"febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
		"fec0::1",
		"fc00::1",
		"fd12:3456:789a::1",
		"fbff:ffff::1",
		"ff01::1",
		"ff11::1",
		"ff02::1",
		"ff02::2",
		"ff12::1",
		"ff05::1",
		"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
		"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff%z",

		// Invalid inputs
		"",
		" ",
		"abc",
		"1.2.3",
		"1.2.3.4.5",
		".1.2.3",
		"1.2.3.",
		"1..2.3",
		"01.2.3.4",
		"1.02.3.4",
		"1.2.00.4",
		"256.0.0.1",
		"1.2.3.256",
		"1.2.3.4%eth0",
		"1.2.3.4:80",
		"1.2.3.4/24",
		" 1.2.3.4",
		"1.2.3.4 ",
		"+1.2.3.4",
		"-1.2.3.4",
		":::",
		"1::2::3",
		"1:2:3:4:5:6:7",
		"1:2:3:4:5:6:7:8:9",
		"1:2:3:4::5:6:7:8",
		"::1:2:3:4:5:6:7:8",
		"1:2:3:4:5:6::1.2.3.4",
		"1:2:3:4:5:6:7:1.2.3.4",
		"12345::1",
		"::1%",
		"%eth0",
		":1",
		"1:",
		"2001:db8::1g",
		"::ffff:01.2.3.4",
		"::ffff:256.2.3.4",
		"::ffff:1.2.3",
		"::ffff:1.2.3.4.5",
		"::ffff:ab.2.3.4",
		"[::1]",
	}

	var v Vectors

	for _, s := range addrInputs {
		ip, err := netip.ParseAddr(s)
		if err != nil {
			v.Addrs = append(v.Addrs, AddrRow{Input: s, Valid: false})
			continue
		}
		a16 := ip.As16()
		row := AddrRow{
			Input:           s,
			Valid:           true,
			String:          ip.String(),
			Expanded:        ip.StringExpanded(),
			Is4:             ip.Is4(),
			Is6:             ip.Is6(),
			Is4In6:          ip.Is4In6(),
			BitLen:          ip.BitLen(),
			Zone:            ip.Zone(),
			Unmap:           ip.Unmap().String(),
			As16Hex:         hex.EncodeToString(a16[:]),
			AsSliceHex:      hex.EncodeToString(ip.AsSlice()),
			IsUnspecified:   ip.IsUnspecified(),
			IsLoopback:      ip.IsLoopback(),
			IsMulticast:     ip.IsMulticast(),
			IsIfLocalMC:     ip.IsInterfaceLocalMulticast(),
			IsLinkLocalMC:   ip.IsLinkLocalMulticast(),
			IsLinkLocalUC:   ip.IsLinkLocalUnicast(),
			IsGlobalUnicast: ip.IsGlobalUnicast(),
			IsPrivate:       ip.IsPrivate(),
			WithZoneEth0:    ip.WithZone("eth0").String(),
			WithZoneEmpty:   ip.WithZone("").String(),
		}
		if ip.Is4() || ip.Is4In6() {
			a4 := ip.As4()
			row.As4Hex = strPtr(hex.EncodeToString(a4[:]))
		}
		bin, err := ip.MarshalBinary()
		if err != nil {
			panic(err)
		}
		row.MarshalBinHex = hex.EncodeToString(bin)
		if nxt := ip.Next(); nxt.IsValid() {
			row.Next = strPtr(nxt.String())
		}
		if prv := ip.Prev(); prv.IsValid() {
			row.Prev = strPtr(prv.String())
		}
		v.Addrs = append(v.Addrs, row)
	}

	addrCmpPairs := [][2]string{
		{"0.0.0.0", "0.0.0.0"},
		{"0.0.0.0", "0.0.0.1"},
		{"1.2.3.4", "1.2.3.5"},
		{"255.255.255.255", "::"},
		{"1.2.3.4", "::ffff:1.2.3.4"},
		{"::", "::1"},
		{"::", "::%a"},
		{"::%a", "::%b"},
		{"fe80::1", "fe80::1%eth0"},
		{"fe80::1%eth0", "fe80::1%eth1"},
		{"fe80::1%eth1", "fe80::2%eth0"},
		{"::ffff:1.2.3.4", "::ffff:1.2.3.5"},
		{"::ffff:1.2.3.4", "::ffff:1.2.3.4%eth0"},
		{"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff", "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"},
	}
	for _, p := range addrCmpPairs {
		a := netip.MustParseAddr(p[0])
		b := netip.MustParseAddr(p[1])
		v.CompareAddr = append(v.CompareAddr, CompareRow{A: p[0], B: p[1], Cmp: a.Compare(b)})
		if p[0] != p[1] {
			v.CompareAddr = append(v.CompareAddr, CompareRow{A: p[1], B: p[0], Cmp: b.Compare(a)})
		}
	}

	addrPortInputs := []string{
		// Valid
		"1.2.3.4:80",
		"0.0.0.0:0",
		"255.255.255.255:65535",
		"127.0.0.1:080",
		"127.0.0.1:00000",
		"[::]:0",
		"[::1]:443",
		"[2001:db8::1]:8080",
		"[fe80::1%eth0]:22",
		"[::ffff:1.2.3.4]:80",
		"[::ffff:1.2.3.4%eth0]:65535",
		"[0:0:0:0:0:ffff:0102:0304]:53",

		// Invalid
		"",
		"1.2.3.4",
		":80",
		"1.2.3.4:",
		"[1.2.3.4]:80",
		"::1:80",
		"2001:db8::1:80",
		"[::1:80",
		"::1]:80",
		"[]:80",
		"1.2.3.4:65536",
		"1.2.3.4:-1",
		"1.2.3.4:+80",
		"1.2.3.4:abc",
		"1.2.3.4: 80",
		"[::1%]:80",
		"[::1]:",
		"[::1]",
		"01.2.3.4:80",
		"1.2.3.4%eth0:80",
	}
	for _, s := range addrPortInputs {
		ap, err := netip.ParseAddrPort(s)
		if err != nil {
			v.AddrPorts = append(v.AddrPorts, AddrPortRow{Input: s, Valid: false})
			continue
		}
		bin, err := ap.MarshalBinary()
		if err != nil {
			panic(err)
		}
		v.AddrPorts = append(v.AddrPorts, AddrPortRow{
			Input:         s,
			Valid:         true,
			String:        ap.String(),
			Addr:          ap.Addr().String(),
			Port:          ap.Port(),
			MarshalBinHex: hex.EncodeToString(bin),
		})
	}

	apCmpPairs := [][2]string{
		{"1.2.3.4:80", "1.2.3.4:80"},
		{"1.2.3.4:80", "1.2.3.4:443"},
		{"1.2.3.4:443", "1.2.3.5:80"},
		{"255.255.255.255:65535", "[::]:0"},
		{"[::1]:80", "[::1%lo0]:80"},
		{"[fe80::1%eth0]:80", "[fe80::1%eth0]:81"},
	}
	for _, p := range apCmpPairs {
		a := netip.MustParseAddrPort(p[0])
		b := netip.MustParseAddrPort(p[1])
		v.CompareAddrPort = append(v.CompareAddrPort, CompareRow{A: p[0], B: p[1], Cmp: a.Compare(b)})
		if p[0] != p[1] {
			v.CompareAddrPort = append(v.CompareAddrPort, CompareRow{A: p[1], B: p[0], Cmp: b.Compare(a)})
		}
	}

	prefixInputs := []string{
		// Valid
		"0.0.0.0/0",
		"10.0.0.0/8",
		"192.168.1.55/24",
		"192.168.1.55/32",
		"255.255.255.255/1",
		"::/0",
		"::1/128",
		"2001:db8::1/32",
		"2001:db8:abcd:ef01::1/64",
		"::ffff:192.0.2.128/96",
		"::ffff:192.0.2.128/120",
		"::ffff:192.0.2.128/128",
		"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff/1",

		// Invalid
		"",
		"1.2.3.4",
		"1.2.3.4/",
		"/24",
		"1.2.3.4/33",
		"1.2.3.4/-1",
		"1.2.3.4/+8",
		"1.2.3.4/08",
		"1.2.3.4/00",
		"1.2.3.4/abc",
		"01.2.3.4/24",
		"::/129",
		"::/-1",
		"::/064",
		"fe80::1%eth0/64",
		"::1%lo/128",
		"1.2.3.4/24/8",
	}
	for _, s := range prefixInputs {
		p, err := netip.ParsePrefix(s)
		if err != nil {
			v.Prefixes = append(v.Prefixes, PrefixRow{Input: s, Valid: false})
			continue
		}
		bin, err := p.MarshalBinary()
		if err != nil {
			panic(err)
		}
		v.Prefixes = append(v.Prefixes, PrefixRow{
			Input:         s,
			Valid:         true,
			String:        p.String(),
			Addr:          p.Addr().String(),
			Bits:          p.Bits(),
			IsSingleIP:    p.IsSingleIP(),
			Masked:        p.Masked().String(),
			MarshalBinHex: hex.EncodeToString(bin),
		})
	}

	addrPrefixCases := []struct {
		addr string
		bits int
	}{
		{"192.168.1.55", 0},
		{"192.168.1.55", 1},
		{"192.168.1.55", 24},
		{"192.168.1.55", 32},
		{"192.168.1.55", -1},
		{"192.168.1.55", 33},
		{"2001:db8:abcd:ef01::1", 0},
		{"2001:db8:abcd:ef01::1", 32},
		{"2001:db8:abcd:ef01::1", 64},
		{"2001:db8:abcd:ef01::1", 128},
		{"2001:db8:abcd:ef01::1", -1},
		{"2001:db8:abcd:ef01::1", 129},
		{"fe80::1%eth0", 64},
		{"::ffff:192.0.2.128", 96},
		{"::ffff:192.0.2.128", 120},
		{"::ffff:192.0.2.128%eth0", 120},
	}
	for _, c := range addrPrefixCases {
		ip := netip.MustParseAddr(c.addr)
		p, err := ip.Prefix(c.bits)
		if err != nil {
			v.AddrPrefixes = append(v.AddrPrefixes, AddrPrefixRow{Addr: c.addr, Bits: c.bits, Valid: false})
		} else {
			v.AddrPrefixes = append(v.AddrPrefixes, AddrPrefixRow{Addr: c.addr, Bits: c.bits, Valid: true, Result: p.String()})
		}
	}

	containsCases := [][2]string{
		{"0.0.0.0/0", "1.2.3.4"},
		{"0.0.0.0/0", "::1"},
		{"192.168.1.0/24", "192.168.1.0"},
		{"192.168.1.0/24", "192.168.1.255"},
		{"192.168.1.0/24", "192.168.2.0"},
		{"192.168.1.55/24", "192.168.1.1"},
		{"192.168.1.55/32", "192.168.1.55"},
		{"192.168.1.55/32", "192.168.1.56"},
		{"192.168.1.0/24", "::ffff:192.168.1.1"},
		{"::ffff:192.168.1.0/120", "192.168.1.1"},
		{"::ffff:192.168.1.0/120", "::ffff:192.168.1.99"},
		{"::ffff:192.168.1.0/120", "::ffff:192.168.1.99%eth0"},
		{"::/0", "::"},
		{"::/0", "2001:db8::1"},
		{"::/0", "fe80::1%eth0"},
		{"::/0", "1.2.3.4"},
		{"2001:db8::/32", "2001:db8:ffff::1"},
		{"2001:db8::/32", "2001:db9::1"},
		{"2001:db8::1/32", "2001:db8::2"},
		{"fe80::/10", "fe80::1"},
		{"fe80::/10", "fe80::1%eth0"},
		{"2001:db8::1/128", "2001:db8::1"},
		{"2001:db8::1/128", "2001:db8::2"},
	}
	for _, c := range containsCases {
		p := netip.MustParsePrefix(c[0])
		ip := netip.MustParseAddr(c[1])
		v.Contains = append(v.Contains, ContainsRow{Prefix: c[0], Addr: c[1], Contains: p.Contains(ip)})
	}

	overlapsCases := [][2]string{
		{"0.0.0.0/0", "192.168.1.0/24"},
		{"0.0.0.0/0", "::/0"},
		{"10.0.0.0/8", "10.1.2.0/24"},
		{"10.1.2.0/24", "10.0.0.0/8"},
		{"10.0.0.0/8", "11.0.0.0/8"},
		{"192.168.1.55/24", "192.168.1.100/24"},
		{"192.168.1.0/25", "192.168.1.128/25"},
		{"192.168.1.0/24", "192.168.1.128/25"},
		{"192.168.1.0/24", "::ffff:192.168.1.0/120"},
		{"::/0", "2001:db8::/32"},
		{"2001:db8::/32", "2001:db8:1::/48"},
		{"2001:db8::/32", "2001:db9::/32"},
		{"2001:db8::1/32", "2001:db8::2/32"},
		{"::ffff:0.0.0.0/96", "::ffff:192.168.1.0/120"},
		{"fe80::/64", "fe80::1/128"},
	}
	for _, c := range overlapsCases {
		a := netip.MustParsePrefix(c[0])
		b := netip.MustParsePrefix(c[1])
		v.Overlaps = append(v.Overlaps, OverlapsRow{A: c[0], B: c[1], Overlaps: a.Overlaps(b)})
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(v); err != nil {
		fmt.Fprintf(os.Stderr, "encode error: %v\n", err)
		os.Exit(1)
	}
}
