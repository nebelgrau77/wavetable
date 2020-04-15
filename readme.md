# simple sine/cosine for embedded use

This is by no means supposed to compete with the fantastic MicroMath :) 
It's just an exercise in writing a simple library. 

Contains two functions: sin_getter and cos_getter, that obtain sine and cosine values for a given angle
from a table precomputed in NumPy. 

Angle has to be an integer value, expressed in degrees. 

Can be useful for code that needs just that, e.g. an analog clock or some rotation on a tiny display.

