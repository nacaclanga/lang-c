# As rust_peg technically doesn't support type, parameters, we need to fix it here

s/< 'input > (/<'input, T: Name>(/g;