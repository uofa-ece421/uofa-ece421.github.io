# Ownership

Ownership is the feature that allows Rust to make guarantees about memory
safety and integrity without compromising performance.

The famous _borrow checker_ runs at compile time, and enforces Rust's ownership
rules to ensure safe and fast access to data.

<h1 style="font-size: 4rem; color: #ff0000">
<p style="text-align:center"><u>Ownership Rules</u></p>
</h1>

<h1 style="font-size: 3rem; color: #aa0000">
<ul>
<li>Every value has a variable called its <em>owner</em></li>
<li>There is only one owner at a time</li>
<li>When the owner's lifetime ends, the value is released</li>
</ul>
</h1>

