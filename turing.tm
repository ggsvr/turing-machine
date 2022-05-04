b : None =  P(a), R, P(a), R, P(0), R, R, P(0), L, L -> o

o : 1    =  R, P(x), L, L, L                         -> o
o : 0    =                                           -> q

q : Any  =  R, R                                     -> q
q : None =  P(1), L                                  -> p

p : x    = E, R                                      -> q
p : a    = R                                         -> f
p : None = L, L                                      -> p

f : Any  = R, R                                      -> f
f : None = P(0), L, L                                -> o