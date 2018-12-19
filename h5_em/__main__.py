"""
MIT License

Copyright (c) 2018 Olga Laviagina

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""
from matplotlib import pyplot
from matplotlib.animation import FuncAnimation
from numpy import random, linspace, zeros_like, array, zeros
from em import gaussian, em_step, likelihood

fig = pyplot.figure()
ax = fig.add_subplot(111)

# Generate points of two classes with normal distribution with random parameters
group1 = (random.randn(random.randint(10, 100)) * (1 + 10 * random.rand() ** 2) +
    random.rand() * 10).tolist()
group2 = (random.randn(random.randint(10, 100)) * (1 + 10 * random.rand() ** 2) +
    random.rand() * 10).tolist()
points = group1 + group2

"""
Generate random probabilities for each point to belong to each class:
`alphas[0]` is a probability of each point to belong to the first class
`alphas[1]` is a probability of each point to belong to the second class
"""
alphas = [random.rand(len(points))]
alphas.append(1 - alphas[0])

bins = linspace(-30, 30, 100)
"""
For each point `colors` contains a list with length 3, where
the first element is a probability that this point belongs to the first class (red channel),
the second element is zero (green channel) and
the third element is a probability that this point belongs to the third class (blue channel).
"""
colors = zeros((len(points), 3), dtype="float64")
colors[:, 0] = alphas[0]
colors[:, 2] = alphas[1]
pyplot.hist(points, bins, alpha=0.3)
pyplot.scatter(points, zeros_like(points) + 0.25, c=colors, marker='|', alpha=0.75, s=200)

def animate(i):
    """
    Each time the function is called,
    all probabilities and parameters are updated in `em_step`
    and dots are redrawn with updated colors
    """
    ax.clear()
    global alphas, q, mu, sigma
    alphas, q, mu, sigma = em_step(points, alphas)
    colors[:, 0] = alphas[0]
    colors[:, 2] = alphas[1]
    pyplot.hist(points, bins, alpha=0.3)
    pyplot.scatter(points, zeros_like(points) + 0.25, c=colors, marker='|', alpha=0.75, s=200)
    likelihood(q, points, mu, sigma)

anim = FuncAnimation(fig, animate, interval=200)
pyplot.show()
