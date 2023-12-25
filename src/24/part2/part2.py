from sympy.solvers import solve
from sympy import Symbol

xs = Symbol('xs')
ys = Symbol('ys')
zs = Symbol('zs')
xv = Symbol('xv')
yv = Symbol('yv')
zv = Symbol('zv')
t1 = Symbol('t1')
t2 = Symbol('t2')
t3 = Symbol('t3')

ans = solve([315268300752660 - 11*t1 - xs - xv*t1, 284016300325583 + 23*t1 - ys - yv*t1, 407533418983227 - 52*t1 - zs - zv*t1,
             393927681060873 - 239*t2 - xs - xv*t2, 429508206398995 - 271*t2 - ys - yv*t2, 348027409734393 - 115*t2 - zs - zv*t2,
             279975598233486 - 37*t3 - xs - xv*t3, 285305766984543 - 9*t3 - ys - yv*t3, 322446398749056 - 83*t3 - zs - zv*t3],
            [xs, ys, zs, t1, t2, t3, xv, yv, zv])
xyz = list(ans[0][0:3])
print(sum(xyz))
