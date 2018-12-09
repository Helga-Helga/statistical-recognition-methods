# MIT License
#
# Copyright (c) 2018 Olga Laviagina
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
from numpy.linalg import eig, eigvals
from numpy import array, zeros
from math import degrees, acos
from matplotlib.patches import Ellipse

def train_on_dots(inside, outside, a=None):
    if a is None:
        a = zeros(6)
    for x, y in inside:
        row = get_vector_in_new_space(x, y)
        if a.dot(row) >= 0:
            a -= row
    for x, y in outside:
        row = get_vector_in_new_space(x, y)
        if a.dot(row) <= 0:
            a += row
    return a

def train_on_eig(a):
    matrix = get_matrix(a)
    while not is_positive_definite(matrix):
        vector = get_wrong_eigvector(matrix)
        row = get_vector_in_new_space(vector[0], vector[1], 0)
        a += row
        matrix = get_matrix(a)
    return a

def get_vector_in_new_space(x, y, not_eigen=1):
    return array([x**2, y**2, x * not_eigen, y * not_eigen, x * y, 1 * not_eigen])

def is_positive_definite(matrix):
    return (eigvals(matrix) > 0).all()

def get_wrong_eigvector(matrix):
    v, w = eig(matrix)
    return w[:, v <= 0][:, 0].tolist()

def get_matrix(a):
    return array([[a[0], a[4] / 2.], [a[4] / 2., a[1]]])
