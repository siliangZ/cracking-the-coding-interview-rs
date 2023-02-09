# Intro
It's a repo to record my solutions to the problems in \<\<Cracking the coding interview>>

# Bit manipulation
## two's complement representation of negative number
## arithmetic vs logical right shift 
* arithmetic: we keep the sign bit and only shift the value and fill the new bit with the value of sign bit. In rust, the shift is always arithmetic shift
* logical: we shift the bit and put 0 at the most significant bit.
