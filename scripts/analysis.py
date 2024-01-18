from math import log

class A(tuple):
    @classmethod
    def new(cls,n):
        return cls((n/log(n), 1/log(n)))
    def times(self, k):
        return A(i*k for i in self)
    def __truediv__(self, x):
        return A(i/j for i,j in zip(self,x))

a = A.new

if __name__ == "__main__":
    d = a(2)
    print("|n| cmp| swap| all|")
    print("|----|----|----|----|")
    for i in range(3, 11):
        r, s = a(i) / d
        print(f"|{i}| {r:#.5f}| {s:#.5f}| {r*s:#.5f}|")
