
# The application of Gibbs sampler to the problem of horizontal and vertical lines

## The problem:
- We have an image <img src="/h2_gibbs_sampler/tex/19e3f7018228f8a8c6559d0ea5500aa2.svg?invert_in_darkmode&sanitize=true" align=middle width=10.747741949999991pt height=23.488575000000026pt/> of size <img src="/h2_gibbs_sampler/tex/205995f88b807b2f5268f7ef4053f049.svg?invert_in_darkmode&sanitize=true" align=middle width=44.39116769999999pt height=19.1781018pt/>,   where each pixel is white or black (<img src="/h2_gibbs_sampler/tex/29632a9bf827ce0200454dd32fc3be82.svg?invert_in_darkmode&sanitize=true" align=middle width=8.219209349999991pt height=21.18721440000001pt/> or <img src="/h2_gibbs_sampler/tex/034d0a6be0424bffe9a6e7ac9236c0f5.svg?invert_in_darkmode&sanitize=true" align=middle width=8.219209349999991pt height=21.18721440000001pt/>)
- <img src="/h2_gibbs_sampler/tex/0e51a2dede42189d77627c4d742822c3.svg?invert_in_darkmode&sanitize=true" align=middle width=14.433101099999991pt height=14.15524440000002pt/> is **WIDTH** and <img src="/h2_gibbs_sampler/tex/55a049b8f161ae7cfeb0197d75aff967.svg?invert_in_darkmode&sanitize=true" align=middle width=9.86687624999999pt height=14.15524440000002pt/> is **HEIGHT**
- At the beginning all the pixels are white:   **image[i][j] = 0** for all **i** and all **j**
- Each row is shaded (or filled) with <img src="/h2_gibbs_sampler/tex/cde2d598001a947a6afd044a43d15629.svg?invert_in_darkmode&sanitize=true" align=middle width=21.00464354999999pt height=21.18721440000001pt/> probability:  each pixel in the horizontal line becomes black
- <img src="/h2_gibbs_sampler/tex/97a36d5ccc9f08d2cd01b1f93f4b899b.svg?invert_in_darkmode&sanitize=true" align=middle width=16.77138869999999pt height=27.91243950000002pt/> is a state of row <img src="/h2_gibbs_sampler/tex/77a3b857d53fb44e33b53e4c8b68351a.svg?invert_in_darkmode&sanitize=true" align=middle width=5.663225699999989pt height=21.68300969999999pt/>
	- if row <img src="/h2_gibbs_sampler/tex/77a3b857d53fb44e33b53e4c8b68351a.svg?invert_in_darkmode&sanitize=true" align=middle width=5.663225699999989pt height=21.68300969999999pt/> is shaded, than <img src="/h2_gibbs_sampler/tex/85f1c09356d05fa5a62b6fd6586280cf.svg?invert_in_darkmode&sanitize=true" align=middle width=47.73014399999999pt height=27.91243950000002pt/> (**filled_row[i] = 1**)
	- if row <img src="/h2_gibbs_sampler/tex/77a3b857d53fb44e33b53e4c8b68351a.svg?invert_in_darkmode&sanitize=true" align=middle width=5.663225699999989pt height=21.68300969999999pt/> is not shaded, than <img src="/h2_gibbs_sampler/tex/72df4e2eae02dbdfa458cc320caf5692.svg?invert_in_darkmode&sanitize=true" align=middle width=47.73014399999999pt height=27.91243950000002pt/> (**filled_row[i] = 0**)
- Each column is shaded with <img src="/h2_gibbs_sampler/tex/cde2d598001a947a6afd044a43d15629.svg?invert_in_darkmode&sanitize=true" align=middle width=21.00464354999999pt height=21.18721440000001pt/> probability:   each pixel in the column becomes black
- <img src="/h2_gibbs_sampler/tex/b3d234448f64691462f80ccc552519f0.svg?invert_in_darkmode&sanitize=true" align=middle width=16.06362284999999pt height=22.831056599999986pt/> is a state of column <img src="/h2_gibbs_sampler/tex/36b5afebdba34564d884d347484ac0c7.svg?invert_in_darkmode&sanitize=true" align=middle width=7.710416999999989pt height=21.68300969999999pt/>
	- if column <img src="/h2_gibbs_sampler/tex/36b5afebdba34564d884d347484ac0c7.svg?invert_in_darkmode&sanitize=true" align=middle width=7.710416999999989pt height=21.68300969999999pt/> is shaded, then <img src="/h2_gibbs_sampler/tex/1df0a56024f3c2db6f77bea500a7d18d.svg?invert_in_darkmode&sanitize=true" align=middle width=47.022378149999994pt height=22.831056599999986pt/> (**filled_column[j] = 1**)
	- if row <img src="/h2_gibbs_sampler/tex/77a3b857d53fb44e33b53e4c8b68351a.svg?invert_in_darkmode&sanitize=true" align=middle width=5.663225699999989pt height=21.68300969999999pt/> is not shaded, then <img src="/h2_gibbs_sampler/tex/b80ab89879fab8e75bc39397af036438.svg?invert_in_darkmode&sanitize=true" align=middle width=47.022378149999994pt height=22.831056599999986pt/> (**filled_column[j] = 0**)
- Each pixel <img src="/h2_gibbs_sampler/tex/c0b1992d3baef9d56284273ee00bfb59.svg?invert_in_darkmode&sanitize=true" align=middle width=33.46495679999999pt height=24.65753399999998pt/> inverts its color   with probability that is equal to <img src="/h2_gibbs_sampler/tex/9ae7733dac2b7b4470696ed36239b676.svg?invert_in_darkmode&sanitize=true" align=middle width=7.66550399999999pt height=14.15524440000002pt/> (<img src="/h2_gibbs_sampler/tex/9ae7733dac2b7b4470696ed36239b676.svg?invert_in_darkmode&sanitize=true" align=middle width=7.66550399999999pt height=14.15524440000002pt/> is known),
  so that we obtain a noised image.
- The task is to find the most probable set of shaded rows and shaded columns
  of initial image, when only noised image and <img src="/h2_gibbs_sampler/tex/9ae7733dac2b7b4470696ed36239b676.svg?invert_in_darkmode&sanitize=true" align=middle width=7.66550399999999pt height=14.15524440000002pt/> are known.

## Used programming language
The program was written on [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)).

You can find how to install Rust [here](https://doc.rust-lang.org/book/2018-edition/ch01-01-installation.html).

To compile and run the project, type [cargo run](https://doc.rust-lang.org/book/2018-edition/ch01-03-hello-cargo.html) from the package folder in the terminal.

> In this program the initial image and <img src="/h2_gibbs_sampler/tex/9ae7733dac2b7b4470696ed36239b676.svg?invert_in_darkmode&sanitize=true" align=middle width=7.66550399999999pt height=14.15524440000002pt/> are generated randomly.
> In the output you can see:
> - the initial image
> - filled rows and columns (ground truth)
> - the value of <img src="/h2_gibbs_sampler/tex/9ae7733dac2b7b4470696ed36239b676.svg?invert_in_darkmode&sanitize=true" align=middle width=7.66550399999999pt height=14.15524440000002pt/>
> - noised image
> - recognized rows and columns that were filled
> - the number of incorrectly recognized rows and columns

## generate_input_image()
At the beginning we shade each row and each column with <img src="/h2_gibbs_sampler/tex/cde2d598001a947a6afd044a43d15629.svg?invert_in_darkmode&sanitize=true" align=middle width=21.00464354999999pt height=21.18721440000001pt/> probability and obtain a set of shaded rows (**filled_rows**) and a set of shaded columns (**filled_columns**).
Pixel <img src="/h2_gibbs_sampler/tex/5971e2f808332d089165a17966b76126.svg?invert_in_darkmode&sanitize=true" align=middle width=33.46495679999999pt height=24.65753399999998pt/> is black (**image[i][j] = 1**) if at least a horizontal or vertical line passes through it.

## noise_image()
Conditional probability of pixel <img src="/h2_gibbs_sampler/tex/f7c878d863b8740a35bef0abf9083e7c.svg?invert_in_darkmode&sanitize=true" align=middle width=45.59960459999999pt height=24.65753399999998pt/> being of particular color,  when shaded rows and columns are known, is equal to
<p align="center"><img src="/h2_gibbs_sampler/tex/59402d33984ac464c51ff4b7e9b56ecc.svg?invert_in_darkmode&sanitize=true" align=middle width=312.2120529pt height=49.315569599999996pt/></p>

> <img src="/h2_gibbs_sampler/tex/43975bca421b2780b079b6207e7ceacb.svg?invert_in_darkmode&sanitize=true" align=middle width=45.59960459999999pt height=24.65753399999998pt/> is a pixel of noised image

That means that pixel changed color with <img src="/h2_gibbs_sampler/tex/9ae7733dac2b7b4470696ed36239b676.svg?invert_in_darkmode&sanitize=true" align=middle width=7.66550399999999pt height=14.15524440000002pt/> probability and didn't change color with <img src="/h2_gibbs_sampler/tex/ecb696a21591a82341cd44a5149a4b42.svg?invert_in_darkmode&sanitize=true" align=middle width=35.97590369999999pt height=21.18721440000001pt/> probability.
More compact version of the equation:
<p align="center"><img src="/h2_gibbs_sampler/tex/ad01e4ffbca3f6ece3eff4209527363b.svg?invert_in_darkmode&sanitize=true" align=middle width=337.57727685pt height=37.8760371pt/></p>
As pixels are noised independently
<p align="center"><img src="/h2_gibbs_sampler/tex/186483addc03cd8426f16c714f4119ec.svg?invert_in_darkmode&sanitize=true" align=middle width=288.64540155pt height=47.1348339pt/></p>

## gibbs_sampler()
We generate a random set of filled columns: a column is filled with <img src="/h2_gibbs_sampler/tex/cde2d598001a947a6afd044a43d15629.svg?invert_in_darkmode&sanitize=true" align=middle width=21.00464354999999pt height=21.18721440000001pt/> probability.
Then make <img src="/h2_gibbs_sampler/tex/675eeb554f7b336873729327dab98036.svg?invert_in_darkmode&sanitize=true" align=middle width=32.876837399999985pt height=21.18721440000001pt/> iterations of defining the most probable state of horizontal and vertical lines

> It's like coordinate descent: we fix a set of filled columns and find the most probable set of filled rows.
> Then we fix found set of filled rows and find the most probable set of filled columns.

From the <img src="/h2_gibbs_sampler/tex/505c39ef5f8f41738b431c3c0ba939f7.svg?invert_in_darkmode&sanitize=true" align=middle width=32.876837399999985pt height=21.18721440000001pt/>st iteration we memorize each <img src="/h2_gibbs_sampler/tex/08f4ed92f27cec32cdd7a6ecd580f9e7.svg?invert_in_darkmode&sanitize=true" align=middle width=16.438418699999993pt height=21.18721440000001pt/>th and than define the most common state of each line (it is done not to take into account very dependent values).
It would be a result.

## fill_rows()
A set of filled columns <img src="/h2_gibbs_sampler/tex/e72272dbdbde7bea238eb84b405c5788.svg?invert_in_darkmode&sanitize=true" align=middle width=16.89978675pt height=32.16441360000002pt/> is fixed.
Our goal here is to find the most probable state of each row, i.e. to define <img src="/h2_gibbs_sampler/tex/431675e369f79fc8cb953d92c464ad59.svg?invert_in_darkmode&sanitize=true" align=middle width=17.607552599999998pt height=32.16441360000002pt/>.

By definition of conditional probability,
<p align="center"><img src="/h2_gibbs_sampler/tex/4534b2cd4dddd7e1266ee39acfbc50fb.svg?invert_in_darkmode&sanitize=true" align=middle width=410.72460494999996pt height=63.7811526pt/></p>

>As lines were shaded independently,
<p align="center"><img src="/h2_gibbs_sampler/tex/b90cfff45178b64d0908327db020a635.svg?invert_in_darkmode&sanitize=true" align=middle width=133.6697769pt height=32.990165999999995pt/></p>

>Also, probability <img src="/h2_gibbs_sampler/tex/63095e9c415c6357896e6ee4c2793012.svg?invert_in_darkmode&sanitize=true" align=middle width=64.23135674999999pt height=37.80850590000001pt/> is not dependent from <img src="/h2_gibbs_sampler/tex/97a36d5ccc9f08d2cd01b1f93f4b899b.svg?invert_in_darkmode&sanitize=true" align=middle width=16.77138869999999pt height=27.91243950000002pt/>, so let's denote the fraction with this probabilities as <img src="/h2_gibbs_sampler/tex/db0e77b2ab4f495dea1f5c5c08588288.svg?invert_in_darkmode&sanitize=true" align=middle width=16.39974929999999pt height=22.465723500000017pt/>

>With the use of low of total probability for <img src="/h2_gibbs_sampler/tex/9a5e8f17faec7ebef64505044b7f90c0.svg?invert_in_darkmode&sanitize=true" align=middle width=92.78342204999998pt height=37.8085191pt/> we get

<p align="center"><img src="/h2_gibbs_sampler/tex/277ea259080e3978abef13c2110fee04.svg?invert_in_darkmode&sanitize=true" align=middle width=597.6928452pt height=45.477827999999995pt/></p>

> Because of rows independence,
> <p align="center"><img src="/h2_gibbs_sampler/tex/7681432e522b882955a3c74acb19156d.svg?invert_in_darkmode&sanitize=true" align=middle width=273.83124945000003pt height=32.990165999999995pt/></p>
> is a constant, that is moved to <img src="/h2_gibbs_sampler/tex/db0e77b2ab4f495dea1f5c5c08588288.svg?invert_in_darkmode&sanitize=true" align=middle width=16.39974929999999pt height=22.465723500000017pt/>

<p align="center"><img src="/h2_gibbs_sampler/tex/360eb1a1300df50f3c091caab706c03d.svg?invert_in_darkmode&sanitize=true" align=middle width=382.65896955pt height=45.477827999999995pt/></p>

> The probability in the equation above was defined in **noise_image()** section.
Let's substitute it here

<p align="center"><img src="/h2_gibbs_sampler/tex/468b2045bd7dc54030bb456aaa810e4e.svg?invert_in_darkmode&sanitize=true" align=middle width=589.99446165pt height=57.44136255pt/></p>

> Move <img src="/h2_gibbs_sampler/tex/ecb696a21591a82341cd44a5149a4b42.svg?invert_in_darkmode&sanitize=true" align=middle width=35.97590369999999pt height=21.18721440000001pt/> to <img src="/h2_gibbs_sampler/tex/db0e77b2ab4f495dea1f5c5c08588288.svg?invert_in_darkmode&sanitize=true" align=middle width=16.39974929999999pt height=22.465723500000017pt/> as it is a constant and denote
> <p align="center"><img src="/h2_gibbs_sampler/tex/ed897f782d76df45a83e62f66d8292be.svg?invert_in_darkmode&sanitize=true" align=middle width=70.4426184pt height=30.8440539pt/></p>
> to make the equation more compact

<p align="center"><img src="/h2_gibbs_sampler/tex/a9fc3ef9f7330711da9bf040eb969be0.svg?invert_in_darkmode&sanitize=true" align=middle width=474.99369720000004pt height=52.0759239pt/></p>

> Each addend is a composition of <img src="/h2_gibbs_sampler/tex/02713dbceae45e5d6a2d9c20bf3b8271.svg?invert_in_darkmode&sanitize=true" align=middle width=36.17195834999999pt height=14.611911599999981pt/> values <img src="/h2_gibbs_sampler/tex/8f1f34e4e8b80c547dde80544d0ff136.svg?invert_in_darkmode&sanitize=true" align=middle width=122.12530439999998pt height=36.4155132pt/>, where only <img src="/h2_gibbs_sampler/tex/55a049b8f161ae7cfeb0197d75aff967.svg?invert_in_darkmode&sanitize=true" align=middle width=9.86687624999999pt height=14.15524440000002pt/> values depend of <img src="/h2_gibbs_sampler/tex/97a36d5ccc9f08d2cd01b1f93f4b899b.svg?invert_in_darkmode&sanitize=true" align=middle width=16.77138869999999pt height=27.91243950000002pt/>

<p align="center"><img src="/h2_gibbs_sampler/tex/2112b6961ff3c8e55e9220cb6fc1b2eb.svg?invert_in_darkmode&sanitize=true" align=middle width=648.3546696pt height=52.0759239pt/></p>

> The value of sum now doesn't depend of <img src="/h2_gibbs_sampler/tex/97a36d5ccc9f08d2cd01b1f93f4b899b.svg?invert_in_darkmode&sanitize=true" align=middle width=16.77138869999999pt height=27.91243950000002pt/>, so move it to <img src="/h2_gibbs_sampler/tex/db0e77b2ab4f495dea1f5c5c08588288.svg?invert_in_darkmode&sanitize=true" align=middle width=16.39974929999999pt height=22.465723500000017pt/>

<p align="center"><img src="/h2_gibbs_sampler/tex/888f17377326aca7d51eb69bb03cb543.svg?invert_in_darkmode&sanitize=true" align=middle width=260.58845505pt height=47.1348339pt/></p>

So formulas to find expressions for probabilities of shading horizontal line
look like this:
<p align="center"><img src="/h2_gibbs_sampler/tex/61fa1c91fdd2b2aef85ee8e8699dd609.svg?invert_in_darkmode&sanitize=true" align=middle width=335.91632415pt height=47.1348339pt/></p>

<p align="center"><img src="/h2_gibbs_sampler/tex/9ccd2dfdfe3432ab19569127cd1c7d79.svg?invert_in_darkmode&sanitize=true" align=middle width=326.82568875pt height=47.1348339pt/></p>

> As the sum of these probabilities must be equal to <img src="/h2_gibbs_sampler/tex/034d0a6be0424bffe9a6e7ac9236c0f5.svg?invert_in_darkmode&sanitize=true" align=middle width=8.219209349999991pt height=21.18721440000001pt/>,
> <p align="center"><img src="/h2_gibbs_sampler/tex/fe2f262bdc17b051d1a20e6728b3170d.svg?invert_in_darkmode&sanitize=true" align=middle width=97.0592733pt height=35.45589465pt/></p>

Finally,
<p align="center"><img src="/h2_gibbs_sampler/tex/d72d4eb278df1cb514c612cf1980ba4b.svg?invert_in_darkmode&sanitize=true" align=middle width=202.75785254999997pt height=36.09514755pt/></p>

<p align="center"><img src="/h2_gibbs_sampler/tex/ec9969095299ca2a88b121a133823fa8.svg?invert_in_darkmode&sanitize=true" align=middle width=202.75785254999997pt height=36.09514755pt/></p>
The state of row i will be 0 or 1,  depending on which probability is greater.
<p align="center"><img src="/h2_gibbs_sampler/tex/cb799c3af624fee8b180399844e29259.svg?invert_in_darkmode&sanitize=true" align=middle width=354.25650479999996pt height=51.2880489pt/></p>

## fill_columns()
A set of filled rows <img src="/h2_gibbs_sampler/tex/431675e369f79fc8cb953d92c464ad59.svg?invert_in_darkmode&sanitize=true" align=middle width=17.607552599999998pt height=32.16441360000002pt/> is fixed.
Our goal here is to find the most probable state of each column, i.e. to define <img src="/h2_gibbs_sampler/tex/e72272dbdbde7bea238eb84b405c5788.svg?invert_in_darkmode&sanitize=true" align=middle width=16.89978675pt height=32.16441360000002pt/>.

> Here everything is similar to the **fill_rows()**

## count_errors()
Function takes two vectors and counts the number of mismatches between them
(element by element).
