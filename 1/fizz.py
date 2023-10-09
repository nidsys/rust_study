#1에서 100까지의 수를 순서대로 출력하는 프로그램을 작성하시오. 단 3의 배소일때는 숫자 대신 Fizz를 5의 배소일떄는 Buzz를 출력하시오. 3과 5의 공배소일때는 FizzBuzz를 출력하시오

for i in range(1, 101):
  dsp = i
  if i % 3==0 and i % 5==0:
    dsp = 'FizzBUzz'
  elif i % 3 ==0:
    dsp = 'Fizz'
  elif i % 5 ==0:
    dsp = 'Buzz'
  
  print(dsp)