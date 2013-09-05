Title: Problem Set 1 Answers
Author: Wil Thomason

1. User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:23.0) Gecko/20100101 Firefox/23.0
	Mozilla: Indicates that my browser is claiming to be a Mozilla-based user agent, which makes sense, as I'm running Firefox.
	5.0: Version of Mozilla, I assume.
	X11: I'm using the X windowing system, version 11.
	Linux x86_64: I'm running 64-bit Arch Linux.
	rv:23.0: Research indicates that this is the version of Gecko being used by the browser.
	Gecko: The browser is using the Gecko engine.
	20100101: The browser build was made on 01/01/2010 (or so this claims).
	Firefox: I'm using Firefox.
	23.0: It's Firefox version 23.0.
2. Rust is likely yelling at me for trying to modify a global variable because, particularly given that we're using a multi-threaded application, doing so could cause data inconsistencies and other fun errors with concurrency (deadlock, race conditions, etc.)