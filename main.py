from math import factorial, floor

def normalize(x):
    total = sum(x)
    return [float(x) / total for x in x]
	
def enumerate_coalitions(n):
    if n == 0:
        yield []
    else:
        for coalition in enumerate_coalitions(n - 1):
            yield coalition
            yield coalition + [n]
	
def power_index(seats: list[int], threshold=None) -> list[float]:
    """Shapley–Shubik power index."""
    if threshold is None:
        threshold = floor(sum(seats) / 2) + 1
    result = [0] * len(seats)
    for coalition in enumerate_coalitions(len(seats) - 1):
        for pivot in range(len(seats)):
            coalition_seats = sum(seats[(pivot + i) % len(seats)] for i in coalition)
            if (coalition_seats < threshold and threshold <= coalition_seats + seats[pivot]):
                result[pivot] += factorial(len(coalition)) * factorial(len(seats) - len(coalition) - 1)
    return normalize(result)

print(power_index([5, 4, 3, 2,7,10]))