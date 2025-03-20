"""
A docstring.

This docstring has its section in the correct order.

See Also
--------
x : Some function.

Examples
--------
An example
"""


def test_right_order(a):
    """
    Check should pass.

    Long description.

    Parameters
    ----------
    a : any
        An argument.

    Returns
    -------
    any
        The return value.

    See Also
    --------
    b : Another function.

    Examples
    --------
    An example
    """


def test_wrong_order(a):
    """
    Check should fail: "Args" should be before "Returns".

    Long description.

    Returns
    -------
    any
        The return value.

    Parameters
    ----------
    a : any
        An argument.

    Examples
    --------
    An example

    See Also
    --------
    b : Another function.
    """
