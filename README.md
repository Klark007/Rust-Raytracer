# Rust-Raytracer

**Desription:**

This is a GPU raytracer written in Rust. It is based on the books [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) and [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html) from Peter Shirley.

**Features:**
<ul>
  <li> Materials: </li>
  <ul>
    <li> Lambertian </li>
    <li> Metal </li>
    <li> Glass </li>
    <li> Textured Material </li>
  </ul>
  <li> Lights </li>
  <li> Bounding volume hierarchie to speed up ray object intersections </li>
</ul>

**Insturction:**
The programm can be run using cargo. First build may take a bit due to compiling dependencies.

**Pictures:**
![Alt text](/classic.png "Differenet materials showcase")
![Alt text](/earth.png "Textured material")
![Alt text](/cornell box.png "Cornell box")
