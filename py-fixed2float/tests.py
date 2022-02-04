import pytest
import fixed2float as f2f


def test1():
    assert f2f.to_float(53, 10, 6, 4) == 3.3125
