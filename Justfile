# https://just.systems/man/en/

# Args:
# sc_year: snake case year, eg: twenty_five
# year: numerical year, eg: 2025
@incr_year sc_year year:
    sed -i "s#mod twenty.*#mod {{ sc_year }};#" src/main.rs
    sed -i "s#\(use crate::\)twenty_\w*\(.*\)#\1{{ sc_year }}\2#" src/main.rs
    sed -i -E "s/\"2[0-9]{3}\"/\"{{ year }}\"/" src/main.rs
    mkdir src/input/{{ year }}
    touch src/input/{{ year }}/example.txt
    touch src/input/{{ year }}/input_one.txt
    mkdir src/{{ sc_year }}
    echo "mod one;\n\npub use one::*;" > src/{{ sc_year }}/mod.rs
    cp template.rs src/{{ sc_year }}/one.rs

@run day sub test="false":
    cargo run -- -d {{ day }} -s {{ sub }} -t {{ test }}
