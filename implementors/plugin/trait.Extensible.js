(function() {var implementors = {};
implementors["mylib"] = ["impl&lt;'mw, 'server, D&gt; <a class='trait' href='plugin/trait.Extensible.html' title='plugin::Extensible'>Extensible</a> for <a class='struct' href='nickel/request/struct.Request.html' title='nickel::request::Request'>Request</a>&lt;'mw, 'server, D&gt;","impl&lt;'a, D, T&gt; <a class='trait' href='plugin/trait.Extensible.html' title='plugin::Extensible'>Extensible</a> for <a class='struct' href='nickel/response/struct.Response.html' title='nickel::response::Response'>Response</a>&lt;'a, D, T&gt; <span class='where'>where T: 'static + <a class='trait' href='https://doc.rust-lang.org/nightly/core/any/trait.Any.html' title='core::any::Any'>Any</a></span>",];
implementors["nickel"] = ["impl&lt;'mw, 'server, D&gt; <a class='trait' href='plugin/trait.Extensible.html' title='plugin::Extensible'>Extensible</a> for <a class='struct' href='nickel/struct.Request.html' title='nickel::Request'>Request</a>&lt;'mw, 'server, D&gt;","impl&lt;'a, D, T:&nbsp;'static + <a class='trait' href='https://doc.rust-lang.org/nightly/core/any/trait.Any.html' title='core::any::Any'>Any</a>&gt; <a class='trait' href='plugin/trait.Extensible.html' title='plugin::Extensible'>Extensible</a> for <a class='struct' href='nickel/struct.Response.html' title='nickel::Response'>Response</a>&lt;'a, D, T&gt;",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
