# declaration
100 x store
# 100 is pushed onto stack
# x is a variable and is "pinned" onto stack
# pinned means : if the next operation is a read, the value is read, if it is a store, the value of x is updated
# store takes the second item on the stack and "copies" or "moves" it to x
# moved means : the value is moved to x, taking ownership of the second item on stack
# copy means : the value is copied to x, duplicating the value
# in this case (x == 100) and 100 was copied. It is scalar and trivial to copy

100 x store
x a store

# x is pinned with 100
# a is pinned with 100, and x was copied
# x is still available

{ "100", 100 } x store
x a store
# x is pinned with object { "100", 100 }
# a is pinned with the data of x, and x was moved!
# x is no longer available
# dynamically sized content is moved


# functions
# a: int b: int # (5 + ) add5 store
# (5 + ) is pushed onto the stack
# add5 is pinned onto the stack
# store will store the value (5 + )
# () is a fancy way to bundle everything inside, add5 will expand to 5 + when its value is read and not pinned.
# in lisp this is called quotations
10 add5
# pushes 15 onto the stack


# pub
(5 + ) add5 pub
# this is the same as the previous example, however this only happens once and is hoisted.
# invoking another assignment is not possible.
# it is hoisted which effects program execution.

# program execution
# (5 + ) add5 pub in a file called myadd.al
# we can then reuse that function.
myadd use

0 add5 add5
# pushes 0 onto the stack
# add5 then pushes 5 + onto the stack and it is evaluated to 10 which is at the top of the stack
# add5 then pushes 5 + onto the stack and it is evaluated to 15 which is now at the top of the stack

# when myadd use, was brought into scope, everything marked pub is hoisted and can be used. 
# alang does not need a garbage collector because of the rules the language has, the three are.
# - stack based
# - program flow 
# - move semantics 

# myadd file
100 x store
(x +) add pub
(200 x store) set200 pub
#Here
# main program
myadd use


0 add # 0 + 100
set200 # 200 is now stored at x
0 add  # 0 + 100 !!!!!!!!
# when add is called, everything that is not marked as pub is pushed onto the stack


100 [] push


foo (a,b) 
  + 100 +

foo2 (a) 100 +

1000 foo; # undefined as only one argument supplied
100 100 foo2; # 200 with an unused variable

[1 2 3] (2 dup 0 !=) (idx 2 * [] swap push) loop
# so then loop will start by
# 2 2 0 != true
# 2 idx
# 1 2 *
# 2 [] swap push
# [] 2 push
# [ 2 ]
