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
from numpy import sqrt, pi, exp, zeros, array
from itertools import compress

def gaussian(x, mu, sigma):
    return 1. / (sigma * sqrt(2 * pi)) * exp( -(x - mu)**2 / (2 * sigma**2))

def get_groups(points, alphas):
    mask = [alphas[0][i] >= alphas[1][i] for i in range(len(alphas[0]))]
    not_mask = [~x for x in mask]
    group1 = list(compress(points, mask))
    group2 = list(compress(points, not_mask))
    return group1, group2

def update_alphas(q, points, mu, sigma, alphas):
    new_alphas = zeros(shape=(2, len(points)), dtype=float)
    for i in range(len(points)):
        f0 = gaussian(points[i], mu[0], sigma[0])
        f1 = gaussian(points[i], mu[1], sigma[1])
        new_alphas[0][i] = q[0] * f0 / float((q[0] * f1 + q[1] * f1))
        new_alphas[1][i] = q[1] * f1 / float((q[0] * f0 + q[1] * f1))
        if new_alphas[0][i] < 10E-6:
            new_alphas[0][i] = alphas[0][i]
        if new_alphas[1][i] < 10E-6:
            new_alphas[1][i] = alphas[1][i]
    return new_alphas

def update_q(alphas, q):
    new_q = array([0., 0.])
    new_q[0] = sum(alphas[0]) / len(alphas[0])
    new_q[1] = sum(alphas[1]) / len(alphas[1])
    if all(new_q < 10E-6):
        return q
    return new_q

def update_mu_sigma(alphas, points, mu, sigma):
    new_mu = array([0., 0.])
    new_sigma = array([0., 0.])
    new_mu[0] = sum(alphas[0] * points) / sum(alphas[0])
    new_mu[1] = sum(alphas[1] * points) / sum(alphas[1])
    new_sigma[0] = sum(alphas[0] * (points - new_mu[0])**2) / sum(alphas[0])
    new_sigma[1] = sum(alphas[1] * (points - new_mu[1])**2) / sum(alphas[1])
    if all(new_mu < 10E-6):
        mu = new_mu
    if all(new_sigma < 10E-6):
        sigma = new_sigma
    return mu, sigma

def em_step(points, alphas, q, mu, sigma):
    alphas = update_alphas(q, points, mu, sigma, alphas)
    q = update_q(alphas, q)
    mu, sigma = update_mu_sigma(alphas, points, mu, sigma)
    return alphas, q, mu, sigma
