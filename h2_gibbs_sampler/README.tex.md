
# The application of Gibbs sampler to the problem of horizontal and vertical lines

## The problem:
- We have an image $\vec{x}$ of size $m \times n$,   where each pixel is white or black ($0$ or $1$)
- $m$ is **WIDTH** and $n$ is **HEIGHT**
- At the beginning all the pixels are white:   **image[i][j] = 0** for all **i** and all **j**
- Each row is shaded (or filled) with $0.5$ probability:  each pixel in the horizontal line becomes black
- $k_i^h$ is a state of row $i$
	- if row $i$ is shaded, than $k_i^h = 1$ (**filled_row[i] = 1**)
	- if row $i$ is not shaded, than $k_i^h = 0$ (**filled_row[i] = 0**)
- Each column is shaded with $0.5$ probability:   each pixel in the column becomes black
- $k_j^v$ is a state of column $j$
	- if column $j$ is shaded, then $k_j^v = 1$ (**filled_column[j] = 1**)
	- if row $i$ is not shaded, then $k_j^v = 0$ (**filled_column[j] = 0**)
- Each pixel $\left( i, j\right)$ inverts its color   with probability that is equal to $\varepsilon$ ($\varepsilon$ is known),
  so that we obtain a noised image.
- The task is to find the most probable set of shaded rows and shaded columns
  of initial image, when only noised image and $\varepsilon$ are known.

## Used programming language
The program was written on [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)).

You can find how to install Rust [here](https://doc.rust-lang.org/book/2018-edition/ch01-01-installation.html).

To compile and run the project, type [cargo run](https://doc.rust-lang.org/book/2018-edition/ch01-03-hello-cargo.html) from the package folder in the terminal.

> In this program the initial image and $\varepsilon$ are generated randomly.
> In the output you can see the initial image, filled rows and columns (ground truth), the value of $\varepsilon$, noised image, recognized rows and columns that were filled and the output image.
> Also the output contains the number of incorrectly recognized rows and columns.

## generate_input_image()
At the beginning we shade each row and each column with $0.5$ probability and obtain a set of shaded rows (**filled_rows**) and a set of shaded columns (**filled_columns**).
Pixel $\left( i, j \right)$ is black (**image[i][j] = 1**) if at least a horizontal or vertical line passes through it.

## noise_image()
Conditional probability of pixel $x\left( i, j\right)$ being of particular color,  when shaded rows and columns are known, is equal to
$$
p\left(x\left( i, j \right) \, \middle| \,k_i^h, k_j^v\right) =
\begin{cases}
\varepsilon, if \, x\left( i, j \right) = k_i^h \lor k_j^v \\
1 - \varepsilon, in \, other \, cases
\end{cases}
$$

> $x\left(i,j \right)$ is a pixel of noised image

That means that pixel changed color with $\varepsilon$ probability and didn't change color with $1 - \varepsilon$ probability.
More compact version of the equation:
$$p\left( x\left( i, j \right) \, \middle| \, k_i^h, k_j^v\right) =
\frac{\varepsilon}{1 - \varepsilon}^{\left| x\left( i, j\right) - k_i^h \lor k_j^v \right|}
\cdot \left( 1 - \varepsilon\right)$$
As pixels are noised independently
$$p\left(\vec{x} \, \middle| \, \vec{k}^h, \vec{k}^v\right) =
\prod \limits_{i = 1}^n \prod \limits_{j = 1}^m
p\left( x\left( i, j \right) \, \middle| \, k_i^h, k_j^v\right)$$

## gibbs_sampler()
We generate a random set of filled columns: a column is filled with $0.5$ probability.
Then make $1000$ iterations of defining the most probable state of horizontal and vertical lines

> It's like coordinate descent: we fix a set of filled columns and find the most probable set of filled rows.
> Then we fix found set of filled rows and find the most probable set of filled columns.

From the $2001$st iteration we memorize each $30$th and than define the most common state of each line (it is done not to take into account very dependent values).
It would be a result.

## fill_rows()
A set of filled columns $\vec{k}^v$ is fixed.
Our goal here is to find the most probable state of each row, i.e. to define $\vec{k}^h$.

By definition of conditional probability,
$$p\left( k_i^h \, \middle| \, \vec{x}, \vec{k}^v\right) =
\frac{p\left( \vec{x}, \vec{k}^v, k_i^h\right)}{p \left( \vec{x}, \vec{k}^v\right)} =
\frac{p \left( \vec{k}^v, k_i^h\right)}{p \left( \vec{x}, \vec{k}^v\right)}
\cdot p \left( \vec{x}\, \middle| \, \vec{k}^v, k_i^h\right)$$

>As lines were shaded independently,
$$p \left( \vec{k}^v, k_i^h\right) =
\frac{1}{2^{m+1}}$$

>Also, probability $p \left( \vec{x}, \vec{k}^v\right)$ is not dependent from $k_i^h$, so let's denote the fraction with this probabilities as $C_i$

>With the use of low of total probability for $p \left( \vec{x}\, \middle| \, \vec{k}^v, k_i^h\right)$ we get

$$p\left( k_i^h \, \middle| \, \vec{x}, \vec{k}^v\right)=
C_i \sum \limits_{k_1^h} \dotsc \sum \limits_{k_{i-1}^h}
\sum \limits_{k_{i+1}^h} \dotsc \sum \limits_{k_n^h}
p \left(\vec{x}\, \middle| \, \vec{k}^v, k_i^h \right)
\cdot p \left( k_1^h, \dotsc, k_{i-1}^h, k_{i+1}^h, \dotsc, k_n^h\right)$$

> Because of rows independence,
> $$p \left( k_1^h, \dotsc, k_{i-1}^h, k_{i+1}^h, \dotsc, k_n^h\right) =
> \frac{1}{2^{n-1}}$$
> is a constant, that is moved to $C_i$

$$p\left( k_i^h \, \middle| \, \vec{x}, \vec{k}^v\right)=
C_i \sum \limits_{k_1^h} \dotsc \sum \limits_{k_{i-1}^h}
\sum \limits_{k_{i+1}^h} \dotsc \sum \limits_{k_n^h}
p \left(\vec{x} \, \middle| \, \vec{k}^v, k_i^h \right)$$

> The probability in the equation above was defined in **noise_image()** section.
Let's substitute it here

$$p\left( k_i^h \, \middle| \, \vec{x}, \vec{k}^v\right)=
C_i \sum \limits_{k_1^h} \dotsc \sum \limits_{k_{i-1}^h}
\sum \limits_{k_{i+1}^h} \dotsc \sum \limits_{k_n^h}
\prod \limits_{i* = 1}^n \prod \limits_{j* = 1}^m
\left( \frac{\varepsilon}{1 - \varepsilon}\right)^{ \left| x \left(i*, j* \right) - k_{i*}^h \lor k_{j*}^v\right|} \cdot \left( 1 - \varepsilon \right)$$

> Move $1 - \varepsilon$ to $C_i$ as it is a constant and denote
> $$\frac{\varepsilon}{1 - \varepsilon} = \alpha$$
> to make the equation more compact

$$p\left( k_i^h \, \middle| \, \vec{x}, \vec{k}^v\right)=
C_i \sum \limits_{k_1^h} \dotsc \sum \limits_{k_{i-1}^h}
\sum \limits_{k_{i+1}^h} \dotsc \sum \limits_{k_n^h}
\prod \limits_{i* = 1}^n \prod \limits_{j* = 1}^m
\alpha^{\left| x \left(i*, j* \right) - k_{i*}^h \lor k_{j*}^v\right|}$$

> Each addend is a composition of $n \cdot m$ values $\alpha^{\left| x \left(i*, j* \right) - k_{i*}^h \lor k_{j*}^v\right|}$, where only $n$ values depend of $k_i^h$

$$p\left( k_i^h \, \middle| \, \vec{x}, \vec{k}^v\right)=C_i
\prod \limits_{j* = 1}^n
\alpha^{\left| x \left(i, j* \right) - k_i^h \lor k_{j*}^v\right|}
\sum \limits_{k_1^h} \dotsc \sum \limits_{k_{i-1}^h} \sum \limits_{k_{i+1}^h}
\dotsc \sum \limits_{k_n^h} \prod \limits_{i* = 1, \, i* \ne i}^n
\prod \limits_{j = 1}^m
\alpha^{\left| x \left(i*, j* \right) - k_{i*}^h \lor k_{j*}^v\right|}$$

> The value of sum now doesn't depend of $k_i^h$, so move it to $C_i$

$$p\left( k_i^h \, \middle| \, \vec{x}, \vec{k}^v\right)=
C_i \prod \limits_{j = 1}^n
\alpha^{\left| x \left(i, j \right) - k_i^h \lor k_j^v\right|}$$

So formulas to find expressions for probabilities of shading horizontal line
look like this:
$$p \left( k_i^h = 0 \, \middle| \, \vec{x}, \vec{k}^v \right) =
C_i \prod \limits_{j = 1}^n
\alpha^{ \left| x \left(i, j \right) - k_j^v \right|} \equiv C_i \cdot P_1$$

$$p \left( k_i^h = 1 \, \middle| \, \vec{x}, \vec{k}^v \right) =
C_i \prod \limits_{j = 1}^n
\alpha^{ \left| x \left(i, j \right) - 1 \right|} \equiv C_i \cdot P_2$$

> As the sum of these probabilities must be equal to $1$,
> $$C_i = \frac{1}{P_1 + P_2}$$

Finally,
$$p \left( k_i^h = 0 \, \middle| \, \vec{x}, \vec{k}^v \right) =
\frac{P_1}{P_1 + P_2}$$

$$p \left( k_i^h = 1 \, \middle| \, \vec{x}, \vec{k}^v \right) =
\frac{P_2}{P_1 + P_2}$$
The state of row i will be 0 or 1,  depending on which probability is greater.
$$k_i^h =
\begin{cases}
1, if \, p \left( k_i^h = 1 \, \middle| \, \vec{x}, \vec{k}^v \right)  >
p \left( k_i^h = 0 \, \middle| \, \vec{x}, \vec{k}^v \right)  \\
0, otherwise
\end{cases}$$

## fill_columns()
A set of filled rows $\vec{k}^h$ is fixed.
Our goal here is to find the most probable state of each column, i.e. to define $\vec{k}^v$.

> Here everything is similar to the **fill_rows()**

## count_errors()
Function takes two vectors and counts the number of mismatches between them
(element by element).

