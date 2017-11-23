extern crate hex;
extern crate gimli_hash;

use gimli_hash::GimliHash;


#[cfg_attr(feature = "cargo-clippy", allow(string_lit_as_bytes))]
#[test]
fn test_hash() {
    macro_rules! test {
        ( @ $input:expr, $output:expr ) => {
            let mut output = [0; 32];
            let mut hasher = GimliHash::default();
            hasher.input($input.as_bytes());
            hasher.finalize(&mut output);
            assert_eq!(hex::decode($output).unwrap(), output, "{}", $input);
        };
        ( $( $input:expr => $output:expr ),* ) => {
            $(
                test!(@ $input, $output);
            )*
        };
    }

    test!{
        "There's plenty for the both of us, may the best Dwarf win."
            => "4afb3ff784c7ad6943d49cf5da79facfa7c4434e1ce44f5dd4b28f91a84d22c8",
        "If anyone was to ask for my opinion, which I note they're not, I'd say we were taking the long way around."
            => "ba82a16a7b224c15bed8e8bdc88903a4006bc7beda78297d96029203ef08e07c",
        "Speak words we can all understand!"
            => "8dd4d132059b72f8e8493f9afb86c6d86263e7439fc64cbb361fcbccf8b01267",
        "It's true you don't see many Dwarf-women. And in fact, they are so alike in voice and appearance, that they are often mistaken for Dwarf-men.  And this in turn has given rise to the belief that there are no Dwarf-women, and that Dwarves just spring out of holes in the ground! Which is, of course, ridiculous."
            => "ebe9bfc05ce15c73336fc3c5b52b01f75cf619bb37f13bfc7f567f9d5603191a",
        ""
            => "b0634b2c0b082aedc5c0a2fe4ee3adcfc989ec05de6f00addb04b3aaac271f67"
    };
}
