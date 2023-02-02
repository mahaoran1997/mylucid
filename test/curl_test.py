import os
import sys
import time

st = time.perf_counter()
insert_op = 10000
for i in range(insert_op):
  os.system('curl --no-progress-meter -X PUT -d "12asdnfjklahsdluity12o4y589012y34hutf89oq23u8er90d127j3489057hjdf890471u2904fd5y29034jyk589027k8403d5juk8023yd4f890235ku809s2k704957jdf80234k7897jkr0df89k7y380945df7yk802347k890d7jyr890df23jyk40tr823" http://localhost:7020/api/kv/resource_'+str(i)+'2>&1 > tmp.txt')

total_tim = time.perf_counter() - st
print("total running time: " + str(total_tim) + " s\n")
print("throughput: " + str(insert_op/total_tim))