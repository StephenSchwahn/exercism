"""Functions used in preparing Guido's gorgeous lasagna.

Learn about Guido, the creator of the Python language:
https://en.wikipedia.org/wiki/Guido_van_Rossum

This is a module docstring, used to describe the functionality
of a module and its functions and/or classes.
"""

EXPECTED_BAKE_TIME = 40 # minutes
PREPARATION_TIME = 2 # minutes

def bake_time_remaining(elapsed_bake_time): 
    """Calculate the bake time remaining.

    :param elapsed_bake_time: int - baking time already elapsed.
    :return: int - remaining bake time (in minutes) derived from 'EXPECTED_BAKE_TIME'.

    Function that takes the actual minutes the lasagna has been in the oven as
    an argument and returns how many minutes the lasagna still needs to bake
    based on the `EXPECTED_BAKE_TIME`.
    """
    return EXPECTED_BAKE_TIME - elapsed_bake_time

def preparation_time_in_minutes(number_of_layers):
    """Calculate the prep time for a lasanga of x layers.

    :param number_of_layers: int - number of layers in the lasagna
    :return: int - prep time (in minutes) derived from 'PREPARATION_TIME'.

    Function that takes the number of lasagna layers and calculates how many
    minutes are needed to do the prep
    """
    return PREPARATION_TIME * number_of_layers
    
def elapsed_time_in_minutes(number_of_layers, elapsed_bake_time):
    """Calculate the total amount of elapsed time for a lasanga of x layers.
     
    :param number_of_layers: int - number of layers in the lasagna
    :param elapsed_bake_time: int - amount of time we've been in the oven
    :return: int - total cook time.

    Function that takes the number of lasagna layers and time already spent in oven
    and returns the total amount of time spent
    """
    return preparation_time_in_minutes(number_of_layers) + elapsed_bake_time
