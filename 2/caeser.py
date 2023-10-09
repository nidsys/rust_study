def encrypt(text, shift):
  code_a = ord('A')
  code_z = ord('Z')

  result = ""

  #한 문자씩 반복
  for ch in text:
    code = ord(ch)

    if code_a <= code <= code_z:
      #shift
      code = (code - code_a + shift) % 26 + code_a
    
    result += chr(code)

  return result


enc = encrypt("ABCDEF",3)
dec = encrypt(enc,-3)
print(enc,"=>",dec)
