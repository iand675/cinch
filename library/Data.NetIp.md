# Data.NetIp

Immutable IPv4, IPv6, 4-in-6 and zoned addresses, AddrPort pairs and CIDR prefixes with RFC 5952 formatting, containment, overlap and binary codecs.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.NetIp</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.NetIp/source/">View source</a><a href="/packages/search/?q=%2BData.NetIp">Search module</a><a href="/reference-sources/Data.NetIp.cinch" download="Data.NetIp.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Hash](Data.Hash.md), [Data.Int](Data.Int.md), [Data.IpAddr](Data.IpAddr.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (65 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Addr">Addr</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Addr</strong></code></li><li><a class="syn-name" href="#t-AddrPort">AddrPort</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AddrPort</strong></code></li><li><a class="syn-name" href="#t-NetIpError">NetIpError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">NetIpError</strong></code></li><li><a class="syn-name" href="#t-Prefix">Prefix</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Prefix</strong></code></li><li><a class="syn-name" href="#v-addr-as-slice">addr-as-slice</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-as-slice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-addr-as16">addr-as16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-as16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-addr-as4">addr-as4</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-as4</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-addr-bit-len">addr-bit-len</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-bit-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-addr-compare">addr-compare</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-addr-from-ip-addr">addr-from-ip-addr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-from-ip-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-from-slice">addr-from-slice</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-from-slice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-from-u128">addr-from-u128</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-from-u128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-from-u32">addr-from-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-from-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-from16">addr-from16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-from16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-from4">addr-from4</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-from4</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-is-global-unicast">addr-is-global-unicast</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is-global-unicast</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-is-interface-local-multicast">addr-is-interface-local-multicast</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is-interface-local-multicast</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-is-link-local-multicast">addr-is-link-local-multicast</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is-link-local-multicast</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-is-link-local-unicast">addr-is-link-local-unicast</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is-link-local-unicast</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-is-loopback">addr-is-loopback</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is-loopback</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-is-multicast">addr-is-multicast</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is-multicast</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-is-private">addr-is-private</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is-private</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-is-unspecified">addr-is-unspecified</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is-unspecified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-is4">addr-is4</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is4</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-is4-in6">addr-is4-in6</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is4-in6</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-is6">addr-is6</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-is6</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-less">addr-less</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-less</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-addr-marshal-binary">addr-marshal-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-marshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-addr-next">addr-next</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-next</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-port-addr">addr-port-addr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-port-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-port-compare">addr-port-compare</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-port-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-addr-port-from">addr-port-from</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-port-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code></li><li><a class="syn-name" href="#v-addr-port-marshal-binary">addr-port-marshal-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-port-marshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-addr-port-port">addr-port-port</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-port-port</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-addr-port-to-string">addr-port-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-port-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-addr-port-unmarshal-binary">addr-port-unmarshal-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-port-unmarshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code></li><li><a class="syn-name" href="#v-addr-prefix">addr-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code></li><li><a class="syn-name" href="#v-addr-prev">addr-prev</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-prev</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-string-expanded">addr-string-expanded</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-string-expanded</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-addr-to-ip-addr">addr-to-ip-addr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-to-ip-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code></li><li><a class="syn-name" href="#v-addr-to-string">addr-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-addr-unmap">addr-unmap</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-unmap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-unmarshal-binary">addr-unmarshal-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-unmarshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-with-zone">addr-with-zone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-with-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-addr-zone">addr-zone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">addr-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-ipv4-unspecified">ipv4-unspecified</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv4-unspecified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-ipv6-link-local-all-nodes">ipv6-link-local-all-nodes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv6-link-local-all-nodes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-ipv6-link-local-all-routers">ipv6-link-local-all-routers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv6-link-local-all-routers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-ipv6-loopback">ipv6-loopback</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv6-loopback</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-ipv6-unspecified">ipv6-unspecified</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv6-unspecified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-netip-error-message">netip-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">netip-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-parse-addr">parse-addr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-parse-addr-port">parse-addr-port</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-addr-port</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code></li><li><a class="syn-name" href="#v-parse-prefix">parse-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code></li><li><a class="syn-name" href="#v-prefix-addr">prefix-addr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-prefix-bits">prefix-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-prefix-compare">prefix-compare</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-prefix-contains">prefix-contains</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-contains</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-prefix-from">prefix-from</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code></li><li><a class="syn-name" href="#v-prefix-is-single-ip">prefix-is-single-ip</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-is-single-ip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-prefix-marshal-binary">prefix-marshal-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-marshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-prefix-masked">prefix-masked</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-masked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code></li><li><a class="syn-name" href="#v-prefix-overlaps">prefix-overlaps</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-overlaps</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-prefix-to-string">prefix-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-prefix-unmarshal-binary">prefix-unmarshal-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prefix-unmarshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Addr">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Addr</strong></code><span class="doc-sig-links"><a href="#t-Addr" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

An immutable IPv4 or IPv6 address (with optional IPv6 scoped addressing zone).

<details class="doc-instances"><summary>Instances (4)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-AddrPort">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AddrPort</strong></code><span class="doc-sig-links"><a href="#t-AddrPort" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

An immutable IP address paired with a 16-bit port number.

<details class="doc-instances"><summary>Instances (4)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-NetIpError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">NetIpError</strong></code><span class="doc-sig-links"><a href="#t-NetIpError" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Why parsing or constructing an address, address-port, or prefix failed.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-NetIpInvalidAddr"><code><strong class="tok-constructor">NetIpInvalidAddr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a></code></li><li id="k-NetIpInvalidAddrPort"><code><strong class="tok-constructor">NetIpInvalidAddrPort</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a></code></li><li id="k-NetIpInvalidPrefix"><code><strong class="tok-constructor">NetIpInvalidPrefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a></code></li><li id="k-NetIpInvalidPrefixBits"><code><strong class="tok-constructor">NetIpInvalidPrefixBits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a></code></li><li id="k-NetIpInvalidBinary"><code><strong class="tok-constructor">NetIpInvalidBinary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-Prefix">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Prefix</strong></code><span class="doc-sig-links"><a href="#t-Prefix" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

An immutable CIDR network prefix: an address (without zone) and a valid bit length.

<details class="doc-instances"><summary>Instances (4)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NetIp/#instances">Data.NetIp</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-addr-as-slice">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-as-slice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-addr-as-slice" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L573" title="View source at line 573">Source</a></span></div>
<div class="doc-body">

Return the 4-byte representation for IPv4 or 16-byte representation for IPv6 (without zone).

</div>
</article>

<article class="doc-entry" id="v-addr-as16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-as16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-addr-as16" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L564" title="View source at line 564">Source</a></span></div>
<div class="doc-body">

Return the 16-byte network-order representation of the address.
IPv4 addresses are represented as IPv4-mapped IPv6 bytes; IPv6 zones are omitted.

</div>
</article>

<article class="doc-entry" id="v-addr-as4">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-as4</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-addr-as4" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L549" title="View source at line 549">Source</a></span></div>
<div class="doc-body">

Return the 4-byte network-order representation for IPv4 or IPv4-mapped IPv6 addresses.
Returns `None` for non-mapped IPv6 addresses.

</div>
</article>

<article class="doc-entry" id="v-addr-bit-len">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-bit-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-addr-bit-len" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L585" title="View source at line 585">Source</a></span></div>
<div class="doc-body">

Number of bits in the address: `32` for IPv4, `128` for IPv6 (including IPv4-mapped IPv6).

</div>
</article>

<article class="doc-entry" id="v-addr-compare">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-addr-compare" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L605" title="View source at line 605">Source</a></span></div>
<div class="doc-body">

Total order on addresses: IPv4 sorts before IPv6, then by numeric address value,
then by IPv6 zone in byte-lexicographic order.

</div>
</article>

<article class="doc-entry" id="v-addr-from-ip-addr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-from-ip-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-from-ip-addr" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L954" title="View source at line 954">Source</a></span></div>
<div class="doc-body">

Convert a `Data.IpAddr.IpAddr` to a `Data.NetIp.Addr`.

</div>
</article>

<article class="doc-entry" id="v-addr-from-slice">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-from-slice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-from-slice" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L337" title="View source at line 337">Source</a></span></div>
<div class="doc-body">

Construct an IPv4 address from 4 bytes or an IPv6 address from 16 bytes.
Returns `None` for any other buffer length.

</div>
</article>

<article class="doc-entry" id="v-addr-from-u128">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-from-u128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-from-u128" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L324" title="View source at line 324">Source</a></span></div>
<div class="doc-body">

Construct an IPv6 address from a big-endian 128-bit unsigned integer (without zone).

</div>
</article>

<article class="doc-entry" id="v-addr-from-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-from-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-from-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L320" title="View source at line 320">Source</a></span></div>
<div class="doc-body">

Construct an IPv4 address from a big-endian 32-bit unsigned integer.

</div>
</article>

<article class="doc-entry" id="v-addr-from16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-from16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-from16" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L330" title="View source at line 330">Source</a></span></div>
<div class="doc-body">

Construct an IPv6 address from a 16-byte buffer in network byte order.
Returns `None` if the buffer length is not 16. An IPv4-mapped IPv6 byte
sequence stays an IPv6 address; call `addr-unmap` to unwrap it.

</div>
</article>

<article class="doc-entry" id="v-addr-from4">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-from4</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-from4" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L316" title="View source at line 316">Source</a></span></div>
<div class="doc-body">

Construct an IPv4 address from four octets in network byte order.

</div>
</article>

<article class="doc-entry" id="v-addr-is-global-unicast">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is-global-unicast</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is-global-unicast" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L710" title="View source at line 710">Source</a></span></div>
<div class="doc-body">

Report whether the address is a global unicast address (unmapping 4-in-6; excludes
unspecified, IPv4 broadcast `255.255.255.255`, loopback, multicast, and link-local unicast).

</div>
</article>

<article class="doc-entry" id="v-addr-is-interface-local-multicast">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is-interface-local-multicast</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is-interface-local-multicast" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L684" title="View source at line 684">Source</a></span></div>
<div class="doc-body">

Report whether the address is an IPv6 interface-local multicast address (`ff01::/16` family).

</div>
</article>

<article class="doc-entry" id="v-addr-is-link-local-multicast">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is-link-local-multicast</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is-link-local-multicast" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L692" title="View source at line 692">Source</a></span></div>
<div class="doc-body">

Report whether the address is a link-local multicast address (`224.0.0.0/24` or `ff02::/16` family, unmapping 4-in-6).

</div>
</article>

<article class="doc-entry" id="v-addr-is-link-local-unicast">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is-link-local-unicast</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is-link-local-unicast" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L701" title="View source at line 701">Source</a></span></div>
<div class="doc-body">

Report whether the address is a link-local unicast address (`169.254.0.0/16` or `fe80::/10`, unmapping 4-in-6).

</div>
</article>

<article class="doc-entry" id="v-addr-is-loopback">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is-loopback</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is-loopback" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L668" title="View source at line 668">Source</a></span></div>
<div class="doc-body">

Report whether the address is a loopback address (`127.0.0.0/8` or `::1`, unmapping 4-in-6).

</div>
</article>

<article class="doc-entry" id="v-addr-is-multicast">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is-multicast</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is-multicast" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L676" title="View source at line 676">Source</a></span></div>
<div class="doc-body">

Report whether the address is a multicast address (`224.0.0.0/4` or `ff00::/8`, unmapping 4-in-6).

</div>
</article>

<article class="doc-entry" id="v-addr-is-private">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is-private</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is-private" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L724" title="View source at line 724">Source</a></span></div>
<div class="doc-body">

Report whether the address is in RFC 1918 IPv4 private space (`10.0.0.0/8`,
`172.16.0.0/12`, `192.168.0.0/16`) or RFC 4193 IPv6 unique local space (`fc00::/7`),
unmapping 4-in-6 addresses first.

</div>
</article>

<article class="doc-entry" id="v-addr-is-unspecified">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is-unspecified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is-unspecified" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L662" title="View source at line 662">Source</a></span></div>
<div class="doc-body">

Report whether the address is `0.0.0.0` or `::` (without a zone).

</div>
</article>

<article class="doc-entry" id="v-addr-is4">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is4</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is4" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L521" title="View source at line 521">Source</a></span></div>
<div class="doc-body">

Report whether the address is an IPv4 address (false for IPv4-mapped IPv6).

</div>
</article>

<article class="doc-entry" id="v-addr-is4-in6">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is4-in6</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is4-in6" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L533" title="View source at line 533">Source</a></span></div>
<div class="doc-body">

Report whether the address is an IPv4-mapped IPv6 address (`::ffff:0:0/96`).

</div>
</article>

<article class="doc-entry" id="v-addr-is6">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-is6</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-is6" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L527" title="View source at line 527">Source</a></span></div>
<div class="doc-body">

Report whether the address is an IPv6 address (including IPv4-mapped IPv6).

</div>
</article>

<article class="doc-entry" id="v-addr-less">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-less</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-addr-less" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L621" title="View source at line 621">Source</a></span></div>
<div class="doc-body">

Report whether `a` sorts strictly before `b` under `addr-compare`.

</div>
</article>

<article class="doc-entry" id="v-addr-marshal-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-marshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-addr-marshal-binary" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L922" title="View source at line 922">Source</a></span></div>
<div class="doc-body">

Encode an address to binary: 4 big-endian bytes for IPv4, or 16 big-endian bytes
followed by the UTF-8 zone bytes for IPv6.

</div>
</article>

<article class="doc-entry" id="v-addr-next">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-next</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-next" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L628" title="View source at line 628">Source</a></span></div>
<div class="doc-body">

Return the next address in numeric order (preserving any IPv6 zone),
or `None` when incrementing would overflow the address family.

</div>
</article>

<article class="doc-entry" id="v-addr-port-addr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-port-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-port-addr" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1065" title="View source at line 1065">Source</a></span></div>
<div class="doc-body">

Extract the `Addr` component of an `AddrPort`.

</div>
</article>

<article class="doc-entry" id="v-addr-port-compare">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-port-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-addr-port-compare" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1146" title="View source at line 1146">Source</a></span></div>
<div class="doc-body">

Total order on `AddrPort`: sorts by `Addr` first, then by port number.

</div>
</article>

<article class="doc-entry" id="v-addr-port-from">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-port-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code><span class="doc-sig-links"><a href="#v-addr-port-from" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1061" title="View source at line 1061">Source</a></span></div>
<div class="doc-body">

Construct an `AddrPort` from an `Addr` and a 16-bit port number.

</div>
</article>

<article class="doc-entry" id="v-addr-port-marshal-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-port-marshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-addr-port-marshal-binary" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1159" title="View source at line 1159">Source</a></span></div>
<div class="doc-body">

Encode an `AddrPort` to binary: `addr-marshal-binary` followed by the 16-bit port
in little-endian byte order.

</div>
</article>

<article class="doc-entry" id="v-addr-port-port">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-port-port</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-addr-port-port" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1069" title="View source at line 1069">Source</a></span></div>
<div class="doc-body">

Extract the 16-bit port component of an `AddrPort`.

</div>
</article>

<article class="doc-entry" id="v-addr-port-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-port-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-addr-port-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1136" title="View source at line 1136">Source</a></span></div>
<div class="doc-body">

Format an `AddrPort` as `ip:port` for IPv4 or `[ip]:port` for IPv6.

</div>
</article>

<article class="doc-entry" id="v-addr-port-unmarshal-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-port-unmarshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code><span class="doc-sig-links"><a href="#v-addr-port-unmarshal-binary" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1173" title="View source at line 1173">Source</a></span></div>
<div class="doc-body">

Decode an `AddrPort` from the binary format produced by `addr-port-marshal-binary`.

</div>
</article>

<article class="doc-entry" id="v-addr-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code><span class="doc-sig-links"><a href="#v-addr-prefix" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L649" title="View source at line 649">Source</a></span></div>
<div class="doc-body">

Mask the address to its top `bits` bits, stripping any IPv6 zone, to form a canonical `Prefix`.

</div>
</article>

<article class="doc-entry" id="v-addr-prev">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-prev</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-prev" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L639" title="View source at line 639">Source</a></span></div>
<div class="doc-body">

Return the previous address in numeric order (preserving any IPv6 zone),
or `None` when decrementing would underflow the address family.

</div>
</article>

<article class="doc-entry" id="v-addr-string-expanded">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-string-expanded</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-addr-string-expanded" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L896" title="View source at line 896">Source</a></span></div>
<div class="doc-body">

Render an address in expanded form: IPv4 renders normally, while IPv6 (including
IPv4-mapped IPv6) renders all 8 four-digit lowercase hex groups with no `::` compression.

</div>
</article>

<article class="doc-entry" id="v-addr-to-ip-addr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-to-ip-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code><span class="doc-sig-links"><a href="#v-addr-to-ip-addr" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L965" title="View source at line 965">Source</a></span></div>
<div class="doc-body">

Convert a zoneless `Addr` to a `Data.IpAddr.IpAddr`.
Returns `None` if the address carries an IPv6 scoped zone.

</div>
</article>

<article class="doc-entry" id="v-addr-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-addr-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L887" title="View source at line 887">Source</a></span></div>
<div class="doc-body">

Render an address in canonical text form (dotted decimal for IPv4,
`::ffff:a.b.c.d` for IPv4-mapped IPv6, RFC 5952 for IPv6, with `%zone` if present).

</div>
</article>

<article class="doc-entry" id="v-addr-unmap">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-unmap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-unmap" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L540" title="View source at line 540">Source</a></span></div>
<div class="doc-body">

Unwrap an IPv4-mapped IPv6 address to its underlying IPv4 address (stripping any zone).
All other addresses are returned unchanged.

</div>
</article>

<article class="doc-entry" id="v-addr-unmarshal-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-unmarshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-unmarshal-binary" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L937" title="View source at line 937">Source</a></span></div>
<div class="doc-body">

Decode an address from the binary format produced by `addr-marshal-binary`.

</div>
</article>

<article class="doc-entry" id="v-addr-with-zone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-with-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-addr-with-zone" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L598" title="View source at line 598">Source</a></span></div>
<div class="doc-body">

Return the address with the given IPv6 scoped addressing zone (empty removes the zone).
For IPv4 addresses this is a no-op.

</div>
</article>

<article class="doc-entry" id="v-addr-zone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">addr-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-addr-zone" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L591" title="View source at line 591">Source</a></span></div>
<div class="doc-body">

The IPv6 scoped addressing zone, or `""` for IPv4 and zoneless IPv6 addresses.

</div>
</article>

<article class="doc-entry" id="v-ipv4-unspecified">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv4-unspecified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-ipv4-unspecified" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L294" title="View source at line 294">Source</a></span></div>
<div class="doc-body">

The IPv4 unspecified address `0.0.0.0`.

</div>
</article>

<article class="doc-entry" id="v-ipv6-link-local-all-nodes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv6-link-local-all-nodes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-ipv6-link-local-all-nodes" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L306" title="View source at line 306">Source</a></span></div>
<div class="doc-body">

The IPv6 link-local all-nodes multicast address `ff02::1`.

</div>
</article>

<article class="doc-entry" id="v-ipv6-link-local-all-routers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv6-link-local-all-routers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-ipv6-link-local-all-routers" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L311" title="View source at line 311">Source</a></span></div>
<div class="doc-body">

The IPv6 link-local all-routers multicast address `ff02::2`.

</div>
</article>

<article class="doc-entry" id="v-ipv6-loopback">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv6-loopback</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-ipv6-loopback" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L302" title="View source at line 302">Source</a></span></div>
<div class="doc-body">

The IPv6 loopback address `::1`.

</div>
</article>

<article class="doc-entry" id="v-ipv6-unspecified">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv6-unspecified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-ipv6-unspecified" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L298" title="View source at line 298">Source</a></span></div>
<div class="doc-body">

The IPv6 unspecified address `::`.

</div>
</article>

<article class="doc-entry" id="v-netip-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">netip-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-netip-error-message" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

Human-readable diagnostic message for a `NetIpError`.

</div>
</article>

<article class="doc-entry" id="v-parse-addr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-parse-addr" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L349" title="View source at line 349">Source</a></span></div>
<div class="doc-body">

Parse an IPv4 or IPv6 address string (including IPv4-mapped IPv6 and
IPv6 scoped addressing zones such as `fe80::1%eth0`).

</div>
</article>

<article class="doc-entry" id="v-parse-addr-port">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-addr-port</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code><span class="doc-sig-links"><a href="#v-parse-addr-port" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1095" title="View source at line 1095">Source</a></span></div>
<div class="doc-body">

Parse an `ip:port` (IPv4) or `[ip]:port` (IPv6) string into an `AddrPort`.
Performs no name resolution: both address and port must be numeric.

</div>
</article>

<article class="doc-entry" id="v-parse-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code><span class="doc-sig-links"><a href="#v-parse-prefix" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1256" title="View source at line 1256">Source</a></span></div>
<div class="doc-body">

Parse a CIDR prefix string such as `"192.168.1.0/24"` or `"2001:db8::/32"`.
Host bits are preserved (call `prefix-masked` to zero them). IPv6 zones and
leading zeros in the bit length are rejected.

</div>
</article>

<article class="doc-entry" id="v-prefix-addr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-prefix-addr" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1279" title="View source at line 1279">Source</a></span></div>
<div class="doc-body">

Extract the `Addr` stored in a `Prefix` (always zoneless).

</div>
</article>

<article class="doc-entry" id="v-prefix-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-prefix-bits" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1283" title="View source at line 1283">Source</a></span></div>
<div class="doc-body">

Extract the prefix bit length (`0..=32` for IPv4, `0..=128` for IPv6).

</div>
</article>

<article class="doc-entry" id="v-prefix-compare">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-prefix-compare" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1344" title="View source at line 1344">Source</a></span></div>
<div class="doc-body">

Total order on `Prefix`: sorts by `Addr` first (IPv4 before IPv6, then numeric value),
then by prefix bit length.

</div>
</article>

<article class="doc-entry" id="v-prefix-contains">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-contains</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-prefix-contains" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1303" title="View source at line 1303">Source</a></span></div>
<div class="doc-body">

Report whether the network prefix `p` contains `ip`.
Returns `False` if `ip` has an IPv6 zone or belongs to a different address family
(an IPv4-mapped IPv6 address does not match an IPv4 prefix, and vice versa).

</div>
</article>

<article class="doc-entry" id="v-prefix-from">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code><span class="doc-sig-links"><a href="#v-prefix-from" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1221" title="View source at line 1221">Source</a></span></div>
<div class="doc-body">

Construct a `Prefix` from an `Addr` and bit length without masking host bits.
Strips any IPv6 zone from `ip`. Fails with `NetIpInvalidPrefixBits` if `bits`
is outside `[0, addr-bit-len ip]`.

</div>
</article>

<article class="doc-entry" id="v-prefix-is-single-ip">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-is-single-ip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-prefix-is-single-ip" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1287" title="View source at line 1287">Source</a></span></div>
<div class="doc-body">

Report whether the prefix matches exactly one address (`/32` for IPv4 or `/128` for IPv6).

</div>
</article>

<article class="doc-entry" id="v-prefix-marshal-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-marshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-prefix-marshal-binary" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1354" title="View source at line 1354">Source</a></span></div>
<div class="doc-body">

Encode a `Prefix` to binary: `addr-marshal-binary` of its zoneless address followed
by one byte carrying the prefix bit length.

</div>
</article>

<article class="doc-entry" id="v-prefix-masked">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-masked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code><span class="doc-sig-links"><a href="#v-prefix-masked" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1292" title="View source at line 1292">Source</a></span></div>
<div class="doc-body">

Return the prefix in canonical form with all host bits beyond `prefix-bits` zeroed.

</div>
</article>

<article class="doc-entry" id="v-prefix-overlaps">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-overlaps</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-prefix-overlaps" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1320" title="View source at line 1320">Source</a></span></div>
<div class="doc-body">

Report whether two prefixes share at least one IP address.
Returns `False` for prefixes of different address families.

</div>
</article>

<article class="doc-entry" id="v-prefix-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-prefix-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1338" title="View source at line 1338">Source</a></span></div>
<div class="doc-body">

Format a `Prefix` in CIDR notation (`"<ip>/<bits>"`).

</div>
</article>

<article class="doc-entry" id="v-prefix-unmarshal-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prefix-unmarshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code><span class="doc-sig-links"><a href="#v-prefix-unmarshal-binary" title="Permalink">#</a><a href="/packages/stdlib/Data.NetIp/source/#L1365" title="View source at line 1365">Source</a></span></div>
<div class="doc-body">

Decode a `Prefix` from the binary format produced by `prefix-marshal-binary`.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Addr"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Eq_Addr</code></span><div class="doc-child-prose">Structural equality on Addr (distinguishes IPv4 from 4-in-6 and includes IPv6 zone).</div></li>
<li id="i-Eq_AddrPort"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Eq_AddrPort</code></span><div class="doc-child-prose">Structural equality on AddrPort.</div></li>
<li id="i-Eq_NetIpError"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Eq_NetIpError</code></span><div class="doc-child-prose">Structural equality on NetIpError.</div></li>
<li id="i-Eq_Prefix"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Eq_Prefix</code></span><div class="doc-child-prose">Structural equality on Prefix.</div></li>
<li id="i-Hash_Addr"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Hash_Addr</code></span><div class="doc-child-prose">Hash digest on Addr, consistent with Eq.</div></li>
<li id="i-Hash_AddrPort"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Hash_AddrPort</code></span><div class="doc-child-prose">Hash digest on AddrPort, consistent with Eq.</div></li>
<li id="i-Hash_Prefix"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Hash_Prefix</code></span><div class="doc-child-prose">Hash digest on Prefix, consistent with Eq.</div></li>
<li id="i-Ord_Addr"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Ord_Addr</code></span><div class="doc-child-prose">Total order on Addr, matching addr-compare.</div></li>
<li id="i-Ord_AddrPort"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Ord_AddrPort</code></span><div class="doc-child-prose">Total order on AddrPort, matching addr-port-compare.</div></li>
<li id="i-Ord_Prefix"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Ord_Prefix</code></span><div class="doc-child-prose">Total order on Prefix, matching prefix-compare.</div></li>
<li id="i-Show_Addr"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Show_Addr</code></span><div class="doc-child-prose">Render an Addr using addr-to-string.</div></li>
<li id="i-Show_AddrPort"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-AddrPort" title="Data.NetIp.AddrPort">AddrPort</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Show_AddrPort</code></span><div class="doc-child-prose">Render an AddrPort using addr-port-to-string.</div></li>
<li id="i-Show_NetIpError"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-NetIpError" title="Data.NetIp.NetIpError">NetIpError</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Show_NetIpError</code></span><div class="doc-child-prose">Render a NetIpError via its message text.</div></li>
<li id="i-Show_Prefix"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NetIp.Show_Prefix</code></span><div class="doc-child-prose">Render a Prefix using prefix-to-string.</div></li>
</ul>
