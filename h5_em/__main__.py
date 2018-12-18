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
from matplotlib import pyplot
from matplotlib.animation import FuncAnimation
from numpy import random, linspace, zeros_like, array, zeros
from em import gaussian, get_groups, em_step, likelihood_k

fig = pyplot.figure()
ax = fig.add_subplot(111)

mu = array([-1, 1])
sigma = array([1, 1])
group1 = [random.normal(random.rand() * 10, random.rand()**2) for _ in range(random.randint(10, 100))]
group2 = [random.normal(random.rand() * 10, random.rand()**2) for _ in range(random.randint(10, 100))]
points = group1 + group2
alphas = [random.rand(len(points))]
alphas.append(1 - alphas[0])
q = array([random.random(), random.random()])

bins = linspace(-10, 10, 100)
colors = zeros((len(points), 3), dtype="float64")
colors[:, 0] = alphas[0]
colors[:, 2] = alphas[1]
pyplot.hist(points, bins, alpha=0.3)
pyplot.scatter(points, zeros_like(points) + 0.25, c=colors, marker='|', alpha=0.75, s=200)
likelihood_k(q, points, mu, sigma)

def animate(i):
    ax.clear()
    global alphas, q, mu, sigma
    alphas, q, mu, sigma = em_step(points, alphas, q, mu, sigma)
    colors[:, 0] = alphas[0]
    colors[:, 2] = alphas[1]
    pyplot.hist(points, bins, alpha=0.3)
    pyplot.scatter(points, zeros_like(points) + 0.25, c=colors, marker='|', alpha=0.75, s=200)
    likelihood_k(q, points, mu, sigma)

anim = FuncAnimation(fig, animate, interval=2000)
pyplot.show()
