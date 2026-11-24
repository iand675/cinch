# Data.NetIp

Immutable IPv4, IPv6, 4-in-6 and zoned addresses, AddrPort pairs and CIDR prefixes with RFC 5952 formatting, containment, overlap and binary codecs.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.NetIp`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.NetIp.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Hash](Data.Hash.md), [Data.Int](Data.Int.md), [Data.IpAddr](Data.IpAddr.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.IpAddr](Data.IpAddr.md), [Data.NetIp](Data.NetIp.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Immutable IPv4 and IPv6 addresses, address-port pairs, and CIDR prefixes.

Every value of `Addr`, `AddrPort`, or `Prefix` is valid by construction:
text parsing and fallible constructors return `Result NetIpError` or `Option`
rather than carrying an uninitialized sentinel state. IPv4 addresses are
stored as big-endian 32-bit unsigned integers, and IPv6 addresses as
big-endian 128-bit unsigned integers with an optional scoped addressing zone.

Text rendering follows RFC 5952 for IPv6 addresses (lowercase hexadecimal,
compressing the longest run of two or more zero 16-bit groups, with the
earliest run winning ties), renders IPv4-mapped IPv6 addresses (`::ffff:0:0/96`)
in `::ffff:a.b.c.d` form, and appends `%zone` when a zone is present.
Address parsing is strict: IPv4 octets forbid leading zeros, zones are only
permitted on IPv6 addresses and must be non-empty, and prefixes forbid zones
and leading zeros in the prefix bit length.

</details>

# Types and constructors

<a id="type-41646472"></a>

## `Addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Addr</span></code></pre>

An immutable IPv4 or IPv6 address (with optional IPv6 scoped addressing zone).

[Source](/reference-sources/Data.NetIp.cinch) · line 103

<a id="type-41646472506f7274"></a>

## `AddrPort`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">AddrPort</span></code></pre>

An immutable IP address paired with a 16-bit port number.

[Source](/reference-sources/Data.NetIp.cinch) · line 110

<a id="type-4e657449704572726f72"></a>

## `NetIpError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">NetIpError</span></code></pre>

Why parsing or constructing an address, address-port, or prefix failed.

[Source](/reference-sources/Data.NetIp.cinch) · line 62

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NetIpInvalidAddr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NetIpInvalidAddrPort</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NetIpInvalidPrefix</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NetIpInvalidPrefixBits</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NetIpInvalidBinary</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span>)</code></pre>

<a id="type-507265666978"></a>

## `Prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Prefix</span></code></pre>

An immutable CIDR network prefix: an address (without zone) and a valid bit length.

[Source](/reference-sources/Data.NetIp.cinch) · line 115

# Values

<a id="value-616464722d61732d736c696365"></a>

## `addr-as-slice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-as-slice</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the 4-byte representation for IPv4 or 16-byte representation for IPv6 (without zone).

[Source](/reference-sources/Data.NetIp.cinch) · line 573

<a id="value-616464722d61733136"></a>

## `addr-as16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-as16</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the 16-byte network-order representation of the address.
IPv4 addresses are represented as IPv4-mapped IPv6 bytes; IPv6 zones are omitted.

[Source](/reference-sources/Data.NetIp.cinch) · line 564

<a id="value-616464722d617334"></a>

## `addr-as4`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-as4</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Return the 4-byte network-order representation for IPv4 or IPv4-mapped IPv6 addresses.
Returns `None` for non-mapped IPv6 addresses.

[Source](/reference-sources/Data.NetIp.cinch) · line 549

<a id="value-616464722d6269742d6c656e"></a>

## `addr-bit-len`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-bit-len</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Number of bits in the address: `32` for IPv4, `128` for IPv6 (including IPv4-mapped IPv6).

[Source](/reference-sources/Data.NetIp.cinch) · line 585

<a id="value-616464722d636f6d70617265"></a>

## `addr-compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-compare</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Total order on addresses: IPv4 sorts before IPv6, then by numeric address value,
then by IPv6 zone in byte-lexicographic order.

[Source](/reference-sources/Data.NetIp.cinch) · line 605

<a id="value-616464722d66726f6d2d69702d61646472"></a>

## `addr-from-ip-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-from-ip-addr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">IpAddr</span>.<span class="tok-constructor">IpAddr</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>)</code></pre>

Convert a `Data.IpAddr.IpAddr` to a `Data.NetIp.Addr`.

[Source](/reference-sources/Data.NetIp.cinch) · line 954

<a id="value-616464722d66726f6d2d736c696365"></a>

## `addr-from-slice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-from-slice</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>))</code></pre>

Construct an IPv4 address from 4 bytes or an IPv6 address from 16 bytes.
Returns `None` for any other buffer length.

[Source](/reference-sources/Data.NetIp.cinch) · line 337

<a id="value-616464722d66726f6d2d75313238"></a>

## `addr-from-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-from-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>)</code></pre>

Construct an IPv6 address from a big-endian 128-bit unsigned integer (without zone).

[Source](/reference-sources/Data.NetIp.cinch) · line 324

<a id="value-616464722d66726f6d2d753332"></a>

## `addr-from-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-from-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>)</code></pre>

Construct an IPv4 address from a big-endian 32-bit unsigned integer.

[Source](/reference-sources/Data.NetIp.cinch) · line 320

<a id="value-616464722d66726f6d3136"></a>

## `addr-from16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-from16</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>))</code></pre>

Construct an IPv6 address from a 16-byte buffer in network byte order.
Returns `None` if the buffer length is not 16. An IPv4-mapped IPv6 byte
sequence stays an IPv6 address; call `addr-unmap` to unwrap it.

[Source](/reference-sources/Data.NetIp.cinch) · line 330

<a id="value-616464722d66726f6d34"></a>

## `addr-from4`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-from4</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>))))</code></pre>

Construct an IPv4 address from four octets in network byte order.

[Source](/reference-sources/Data.NetIp.cinch) · line 316

<a id="value-616464722d69732d676c6f62616c2d756e6963617374"></a>

## `addr-is-global-unicast`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is-global-unicast</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is a global unicast address (unmapping 4-in-6; excludes
unspecified, IPv4 broadcast `255.255.255.255`, loopback, multicast, and link-local unicast).

[Source](/reference-sources/Data.NetIp.cinch) · line 710

<a id="value-616464722d69732d696e746572666163652d6c6f63616c2d6d756c746963617374"></a>

## `addr-is-interface-local-multicast`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is-interface-local-multicast</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is an IPv6 interface-local multicast address (`ff01::/16` family).

[Source](/reference-sources/Data.NetIp.cinch) · line 684

<a id="value-616464722d69732d6c696e6b2d6c6f63616c2d6d756c746963617374"></a>

## `addr-is-link-local-multicast`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is-link-local-multicast</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is a link-local multicast address (`224.0.0.0/24` or `ff02::/16` family, unmapping 4-in-6).

[Source](/reference-sources/Data.NetIp.cinch) · line 692

<a id="value-616464722d69732d6c696e6b2d6c6f63616c2d756e6963617374"></a>

## `addr-is-link-local-unicast`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is-link-local-unicast</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is a link-local unicast address (`169.254.0.0/16` or `fe80::/10`, unmapping 4-in-6).

[Source](/reference-sources/Data.NetIp.cinch) · line 701

<a id="value-616464722d69732d6c6f6f706261636b"></a>

## `addr-is-loopback`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is-loopback</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is a loopback address (`127.0.0.0/8` or `::1`, unmapping 4-in-6).

[Source](/reference-sources/Data.NetIp.cinch) · line 668

<a id="value-616464722d69732d6d756c746963617374"></a>

## `addr-is-multicast`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is-multicast</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is a multicast address (`224.0.0.0/4` or `ff00::/8`, unmapping 4-in-6).

[Source](/reference-sources/Data.NetIp.cinch) · line 676

<a id="value-616464722d69732d70726976617465"></a>

## `addr-is-private`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is-private</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is in RFC 1918 IPv4 private space (`10.0.0.0/8`,
`172.16.0.0/12`, `192.168.0.0/16`) or RFC 4193 IPv6 unique local space (`fc00::/7`),
unmapping 4-in-6 addresses first.

[Source](/reference-sources/Data.NetIp.cinch) · line 724

<a id="value-616464722d69732d756e737065636966696564"></a>

## `addr-is-unspecified`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is-unspecified</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is `0.0.0.0` or `::` (without a zone).

[Source](/reference-sources/Data.NetIp.cinch) · line 662

<a id="value-616464722d697334"></a>

## `addr-is4`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is4</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is an IPv4 address (false for IPv4-mapped IPv6).

[Source](/reference-sources/Data.NetIp.cinch) · line 521

<a id="value-616464722d6973342d696e36"></a>

## `addr-is4-in6`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is4-in6</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is an IPv4-mapped IPv6 address (`::ffff:0:0/96`).

[Source](/reference-sources/Data.NetIp.cinch) · line 533

<a id="value-616464722d697336"></a>

## `addr-is6`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-is6</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the address is an IPv6 address (including IPv4-mapped IPv6).

[Source](/reference-sources/Data.NetIp.cinch) · line 527

<a id="value-616464722d6c657373"></a>

## `addr-less`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-less</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Report whether `a` sorts strictly before `b` under `addr-compare`.

[Source](/reference-sources/Data.NetIp.cinch) · line 621

<a id="value-616464722d6d61727368616c2d62696e617279"></a>

## `addr-marshal-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-marshal-binary</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode an address to binary: 4 big-endian bytes for IPv4, or 16 big-endian bytes
followed by the UTF-8 zone bytes for IPv6.

[Source](/reference-sources/Data.NetIp.cinch) · line 922

<a id="value-616464722d6e657874"></a>

## `addr-next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-next</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>))</code></pre>

Return the next address in numeric order (preserving any IPv6 zone),
or `None` when incrementing would overflow the address family.

[Source](/reference-sources/Data.NetIp.cinch) · line 628

<a id="value-616464722d706f72742d61646472"></a>

## `addr-port-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-port-addr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">AddrPort</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>)</code></pre>

Extract the `Addr` component of an `AddrPort`.

[Source](/reference-sources/Data.NetIp.cinch) · line 1065

<a id="value-616464722d706f72742d636f6d70617265"></a>

## `addr-port-compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-port-compare</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">AddrPort</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">AddrPort</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Total order on `AddrPort`: sorts by `Addr` first, then by port number.

[Source](/reference-sources/Data.NetIp.cinch) · line 1146

<a id="value-616464722d706f72742d66726f6d"></a>

## `addr-port-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-port-from</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">AddrPort</span>))</code></pre>

Construct an `AddrPort` from an `Addr` and a 16-bit port number.

[Source](/reference-sources/Data.NetIp.cinch) · line 1061

<a id="value-616464722d706f72742d6d61727368616c2d62696e617279"></a>

## `addr-port-marshal-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-port-marshal-binary</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">AddrPort</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode an `AddrPort` to binary: `addr-marshal-binary` followed by the 16-bit port
in little-endian byte order.

[Source](/reference-sources/Data.NetIp.cinch) · line 1159

<a id="value-616464722d706f72742d706f7274"></a>

## `addr-port-port`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-port-port</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">AddrPort</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Extract the 16-bit port component of an `AddrPort`.

[Source](/reference-sources/Data.NetIp.cinch) · line 1069

<a id="value-616464722d706f72742d746f2d737472696e67"></a>

## `addr-port-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-port-to-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">AddrPort</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format an `AddrPort` as `ip:port` for IPv4 or `[ip]:port` for IPv6.

[Source](/reference-sources/Data.NetIp.cinch) · line 1136

<a id="value-616464722d706f72742d756e6d61727368616c2d62696e617279"></a>

## `addr-port-unmarshal-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-port-unmarshal-binary</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">AddrPort</span>))</code></pre>

Decode an `AddrPort` from the binary format produced by `addr-port-marshal-binary`.

[Source](/reference-sources/Data.NetIp.cinch) · line 1173

<a id="value-616464722d707265666978"></a>

## `addr-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-prefix</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span>)))</code></pre>

Mask the address to its top `bits` bits, stripping any IPv6 zone, to form a canonical `Prefix`.

[Source](/reference-sources/Data.NetIp.cinch) · line 649

<a id="value-616464722d70726576"></a>

## `addr-prev`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-prev</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>))</code></pre>

Return the previous address in numeric order (preserving any IPv6 zone),
or `None` when decrementing would underflow the address family.

[Source](/reference-sources/Data.NetIp.cinch) · line 639

<a id="value-616464722d737472696e672d657870616e646564"></a>

## `addr-string-expanded`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-string-expanded</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an address in expanded form: IPv4 renders normally, while IPv6 (including
IPv4-mapped IPv6) renders all 8 four-digit lowercase hex groups with no `::` compression.

[Source](/reference-sources/Data.NetIp.cinch) · line 896

<a id="value-616464722d746f2d69702d61646472"></a>

## `addr-to-ip-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-to-ip-addr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">IpAddr</span>.<span class="tok-constructor">IpAddr</span>))</code></pre>

Convert a zoneless `Addr` to a `Data.IpAddr.IpAddr`.
Returns `None` if the address carries an IPv6 scoped zone.

[Source](/reference-sources/Data.NetIp.cinch) · line 965

<a id="value-616464722d746f2d737472696e67"></a>

## `addr-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-to-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an address in canonical text form (dotted decimal for IPv4,
`::ffff:a.b.c.d` for IPv4-mapped IPv6, RFC 5952 for IPv6, with `%zone` if present).

[Source](/reference-sources/Data.NetIp.cinch) · line 887

<a id="value-616464722d756e6d6170"></a>

## `addr-unmap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-unmap</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>)</code></pre>

Unwrap an IPv4-mapped IPv6 address to its underlying IPv4 address (stripping any zone).
All other addresses are returned unchanged.

[Source](/reference-sources/Data.NetIp.cinch) · line 540

<a id="value-616464722d756e6d61727368616c2d62696e617279"></a>

## `addr-unmarshal-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-unmarshal-binary</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>))</code></pre>

Decode an address from the binary format produced by `addr-marshal-binary`.

[Source](/reference-sources/Data.NetIp.cinch) · line 937

<a id="value-616464722d776974682d7a6f6e65"></a>

## `addr-with-zone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-with-zone</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>))</code></pre>

Return the address with the given IPv6 scoped addressing zone (empty removes the zone).
For IPv4 addresses this is a no-op.

[Source](/reference-sources/Data.NetIp.cinch) · line 598

<a id="value-616464722d7a6f6e65"></a>

## `addr-zone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">addr-zone</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The IPv6 scoped addressing zone, or `""` for IPv4 and zoneless IPv6 addresses.

[Source](/reference-sources/Data.NetIp.cinch) · line 591

<a id="value-697076342d756e737065636966696564"></a>

## `ipv4-unspecified`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ipv4-unspecified</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span></code></pre>

The IPv4 unspecified address `0.0.0.0`.

[Source](/reference-sources/Data.NetIp.cinch) · line 294

<a id="value-697076362d6c696e6b2d6c6f63616c2d616c6c2d6e6f646573"></a>

## `ipv6-link-local-all-nodes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ipv6-link-local-all-nodes</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span></code></pre>

The IPv6 link-local all-nodes multicast address `ff02::1`.

[Source](/reference-sources/Data.NetIp.cinch) · line 306

<a id="value-697076362d6c696e6b2d6c6f63616c2d616c6c2d726f7574657273"></a>

## `ipv6-link-local-all-routers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ipv6-link-local-all-routers</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span></code></pre>

The IPv6 link-local all-routers multicast address `ff02::2`.

[Source](/reference-sources/Data.NetIp.cinch) · line 311

<a id="value-697076362d6c6f6f706261636b"></a>

## `ipv6-loopback`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ipv6-loopback</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span></code></pre>

The IPv6 loopback address `::1`.

[Source](/reference-sources/Data.NetIp.cinch) · line 302

<a id="value-697076362d756e737065636966696564"></a>

## `ipv6-unspecified`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ipv6-unspecified</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span></code></pre>

The IPv6 unspecified address `::`.

[Source](/reference-sources/Data.NetIp.cinch) · line 298

<a id="value-6e657469702d6572726f722d6d657373616765"></a>

## `netip-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">netip-error-message</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Human-readable diagnostic message for a `NetIpError`.

[Source](/reference-sources/Data.NetIp.cinch) · line 75

<a id="value-70617273652d61646472"></a>

## `parse-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-addr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>))</code></pre>

Parse an IPv4 or IPv6 address string (including IPv4-mapped IPv6 and
IPv6 scoped addressing zones such as `fe80::1%eth0`).

[Source](/reference-sources/Data.NetIp.cinch) · line 349

<a id="value-70617273652d616464722d706f7274"></a>

## `parse-addr-port`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-addr-port</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">AddrPort</span>))</code></pre>

Parse an `ip:port` (IPv4) or `[ip]:port` (IPv6) string into an `AddrPort`.
Performs no name resolution: both address and port must be numeric.

[Source](/reference-sources/Data.NetIp.cinch) · line 1095

<a id="value-70617273652d707265666978"></a>

## `parse-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-prefix</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span>))</code></pre>

Parse a CIDR prefix string such as `"192.168.1.0/24"` or `"2001:db8::/32"`.
Host bits are preserved (call `prefix-masked` to zero them). IPv6 zones and
leading zeros in the bit length are rejected.

[Source](/reference-sources/Data.NetIp.cinch) · line 1256

<a id="value-7072656669782d61646472"></a>

## `prefix-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-addr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>)</code></pre>

Extract the `Addr` stored in a `Prefix` (always zoneless).

[Source](/reference-sources/Data.NetIp.cinch) · line 1279

<a id="value-7072656669782d62697473"></a>

## `prefix-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-bits</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Extract the prefix bit length (`0..=32` for IPv4, `0..=128` for IPv6).

[Source](/reference-sources/Data.NetIp.cinch) · line 1283

<a id="value-7072656669782d636f6d70617265"></a>

## `prefix-compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-compare</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Total order on `Prefix`: sorts by `Addr` first (IPv4 before IPv6, then numeric value),
then by prefix bit length.

[Source](/reference-sources/Data.NetIp.cinch) · line 1344

<a id="value-7072656669782d636f6e7461696e73"></a>

## `prefix-contains`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-contains</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Report whether the network prefix `p` contains `ip`.
Returns `False` if `ip` has an IPv6 zone or belongs to a different address family
(an IPv4-mapped IPv6 address does not match an IPv4 prefix, and vice versa).

[Source](/reference-sources/Data.NetIp.cinch) · line 1303

<a id="value-7072656669782d66726f6d"></a>

## `prefix-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-from</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span>)))</code></pre>

Construct a `Prefix` from an `Addr` and bit length without masking host bits.
Strips any IPv6 zone from `ip`. Fails with `NetIpInvalidPrefixBits` if `bits`
is outside `[0, addr-bit-len ip]`.

[Source](/reference-sources/Data.NetIp.cinch) · line 1221

<a id="value-7072656669782d69732d73696e676c652d6970"></a>

## `prefix-is-single-ip`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-is-single-ip</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the prefix matches exactly one address (`/32` for IPv4 or `/128` for IPv6).

[Source](/reference-sources/Data.NetIp.cinch) · line 1287

<a id="value-7072656669782d6d61727368616c2d62696e617279"></a>

## `prefix-marshal-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-marshal-binary</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a `Prefix` to binary: `addr-marshal-binary` of its zoneless address followed
by one byte carrying the prefix bit length.

[Source](/reference-sources/Data.NetIp.cinch) · line 1354

<a id="value-7072656669782d6d61736b6564"></a>

## `prefix-masked`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-masked</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span>)</code></pre>

Return the prefix in canonical form with all host bits beyond `prefix-bits` zeroed.

[Source](/reference-sources/Data.NetIp.cinch) · line 1292

<a id="value-7072656669782d6f7665726c617073"></a>

## `prefix-overlaps`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-overlaps</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Report whether two prefixes share at least one IP address.
Returns `False` for prefixes of different address families.

[Source](/reference-sources/Data.NetIp.cinch) · line 1320

<a id="value-7072656669782d746f2d737472696e67"></a>

## `prefix-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-to-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a `Prefix` in CIDR notation (`"<ip>/<bits>"`).

[Source](/reference-sources/Data.NetIp.cinch) · line 1338

<a id="value-7072656669782d756e6d61727368616c2d62696e617279"></a>

## `prefix-unmarshal-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prefix-unmarshal-binary</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">NetIpError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span>))</code></pre>

Decode a `Prefix` from the binary format produced by `prefix-marshal-binary`.

[Source](/reference-sources/Data.NetIp.cinch) · line 1365

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.NetIp.Eq_Addr` | `Relation.Eq.Eq Data.NetIp.Addr` | Implicit |
| `Data.NetIp.Eq_AddrPort` | `Relation.Eq.Eq Data.NetIp.AddrPort` | Implicit |
| `Data.NetIp.Eq_NetIpError` | `Relation.Eq.Eq Data.NetIp.NetIpError` | Implicit |
| `Data.NetIp.Eq_Prefix` | `Relation.Eq.Eq Data.NetIp.Prefix` | Implicit |
| `Data.NetIp.Hash_Addr` | `Data.Hash.Hash Data.NetIp.Addr` | Implicit |
| `Data.NetIp.Hash_AddrPort` | `Data.Hash.Hash Data.NetIp.AddrPort` | Implicit |
| `Data.NetIp.Hash_Prefix` | `Data.Hash.Hash Data.NetIp.Prefix` | Implicit |
| `Data.NetIp.Ord_Addr` | `Relation.Ord.Ord Data.NetIp.Addr` | Implicit |
| `Data.NetIp.Ord_AddrPort` | `Relation.Ord.Ord Data.NetIp.AddrPort` | Implicit |
| `Data.NetIp.Ord_Prefix` | `Relation.Ord.Ord Data.NetIp.Prefix` | Implicit |
| `Data.NetIp.Show_Addr` | `Text.Show.Show Data.NetIp.Addr` | Implicit |
| `Data.NetIp.Show_AddrPort` | `Text.Show.Show Data.NetIp.AddrPort` | Implicit |
| `Data.NetIp.Show_NetIpError` | `Text.Show.Show Data.NetIp.NetIpError` | Implicit |
| `Data.NetIp.Show_Prefix` | `Text.Show.Show Data.NetIp.Prefix` | Implicit |

**`Data.NetIp.Eq_Addr`**: Structural equality on `Addr` (distinguishes IPv4 from 4-in-6 and includes IPv6 zone).
**`Data.NetIp.Eq_AddrPort`**: Structural equality on `AddrPort`.
**`Data.NetIp.Eq_NetIpError`**: Structural equality on `NetIpError`.
**`Data.NetIp.Eq_Prefix`**: Structural equality on `Prefix`.
**`Data.NetIp.Hash_Addr`**: Hash digest on `Addr`, consistent with `Eq`.
**`Data.NetIp.Hash_AddrPort`**: Hash digest on `AddrPort`, consistent with `Eq`.
**`Data.NetIp.Hash_Prefix`**: Hash digest on `Prefix`, consistent with `Eq`.
**`Data.NetIp.Ord_Addr`**: Total order on `Addr`, matching `addr-compare`.
**`Data.NetIp.Ord_AddrPort`**: Total order on `AddrPort`, matching `addr-port-compare`.
**`Data.NetIp.Ord_Prefix`**: Total order on `Prefix`, matching `prefix-compare`.
**`Data.NetIp.Show_Addr`**: Render an `Addr` using `addr-to-string`.
**`Data.NetIp.Show_AddrPort`**: Render an `AddrPort` using `addr-port-to-string`.
**`Data.NetIp.Show_NetIpError`**: Render a `NetIpError` via its message text.
**`Data.NetIp.Show_Prefix`**: Render a `Prefix` using `prefix-to-string`.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/NetIp.cinch`. Source SHA-256: `0d9f35f9c3a4250bf176f4484e654b3e82662c655f03b2b23c8e00727eb19726`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
