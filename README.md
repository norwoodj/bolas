# bolas

Bolas is portuguese for "balls". Moro em Portugal agora, e escrevi este projeto aqui, então chamei-o "bolas".

This is a dumb little physics simulator where you can click and drag in a slingshot-type motion and
fling balls around a canvas.

This was mostly an excuse to write some rust I could show on my github and to use websockets and the
actix web framework a bit on a side-project.

Right now there are a lot of improvements to be made:

-   Make the physics better, right now the balls stick together at times on collisions
-   Write a better README with a gif of the balls in action

### Building and Developing Locally

You will need cargo installed to build/run locally. To start the server,
run the following, then navigate to http://localhost:23080 in your browser.

```
make run
```
