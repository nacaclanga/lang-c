# Because rust_peg technically doesn't support type parameters, we need to fix them here.

s/< 'input > (/<'input, T: Name>(/g;
