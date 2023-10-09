for i in range(1,10):
  a = ["{:3}".format(i*k) for k in range(1,10)]
  print(",".join(a))