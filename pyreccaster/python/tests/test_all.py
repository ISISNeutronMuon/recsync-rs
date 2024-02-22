import pytest
import pyreccaster


def test_sum_as_string():
    assert pyreccaster.sum_as_string(1, 1) == "2"
