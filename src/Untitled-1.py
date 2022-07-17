#!/usr/bin/env python3
import math
import random
import time
from quiz.arcsine import arcsine_as_sum


EPSILON = 1e-6
CHECK_TRIES = 10_000
LOAD_TRIES = 10_000_000


def random_x():
    # The expansion sum converges too long for an argument close (in modulo) to one.
    # So we'll cut you some corners.
    return random.random() * 1.8 - 0.9


def check_correctness():
    for n in range(CHECK_TRIES):
        x = random_x()
        a = math.asin(x)
        b = arcsine_as_sum(x, EPSILON)
        if not math.isclose(a, b, abs_tol=EPSILON):
            print(f'asin({x}) = {a}, but calculated as {b}, diff is {a - b}')
            return False
    return True


def check_speed():
    print('Generating test set...')
    data = [random_x() for _ in range(LOAD_TRIES)]

    print('Testing...')
    t_start = time.perf_counter()
    for x in data:
        _ = arcsine_as_sum(x, EPSILON)
    t_end = time.perf_counter()

    cps = LOAD_TRIES / (t_end - t_start)
    print('Calls per second:', cps)


def main():
    ok = check_correctness()
    if ok:
        check_speed()


main()
