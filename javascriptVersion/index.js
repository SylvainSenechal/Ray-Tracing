const canvas = document.getElementById('canvas')
const ctx = canvas.getContext('2d')
ctx.canvas.width = window.innerWidth
ctx.canvas.height = window.innerHeight
ctx.font = '15px serif'


const DRAWING_OFFSET_X = 50
const DRAWING_OFFSET_Y = 50

const SAMPLE_PER_PIXEL = 4
const IMAGE_WIDTH = 400
const ASPECT_RATIO = 16.0 / 9.0
const IMAGE_HEIGHT = Math.floor(IMAGE_WIDTH / ASPECT_RATIO)


class Vec3 {
  constructor(x, y, z) {
    this.x = x
    this.y = y
    this.z = z
  }

  add = (vec) => new Vec3(this.x + vec.x, this.y + vec.y, this.z + vec.z)
  sub = (vec) => new Vec3(this.x - vec.x, this.y - vec.y, this.z - vec.z)
  mul = (val) => new Vec3(this.x * val, this.y * val, this.z * val)
  div = (val) => new Vec3(this.x / val, this.y / val, this.z / val)

  dot = (vec) => this.x * vec.x + this.y * vec.y + this.z * vec.z
  lengthSquared = () => this.x * this.x + this.y * this.y + this.z * this.z
  length = () => Math.sqrt(this.lengthSquared())
  unitVector = () => this.div(this.length())
}


class Camera {
  constructor() {
    this.aspectRatio = 16.0 / 9.0
    this.viewportHeight = 2.0
    this.viewportWidth = this.aspectRatio * this.viewportHeight
    this.focalLength = 1.0

    this.origin = new Vec3(0, 0, 0)
    this.horizontal = new Vec3(this.viewportWidth, 0, 0)
    this.vertical = new Vec3(0.0, this.viewportHeight, 0.0)
    this.lowerLeftCorner = this.origin.sub(this.horizontal.div(2)).sub(this.vertical.div(2)).sub(new Vec3(0, 0, this.focalLength))
  }

  getRay = (u, v) => new Ray(this.origin, this.lowerLeftCorner.add(this.horizontal.mul(u)).add(this.vertical.mul(v)).sub(this.origin))
}

class Color {
  constructor(r, g, b) {
    this.r = r
    this.g = g
    this.b = b
  }

  draw = (ctx, x, y) => {
    this.divThis(SAMPLE_PER_PIXEL)
    ctx.fillStyle = `rgb(${this.clamp(this.r, 0.0, 0.999) * 256}, ${this.clamp(this.g, 0.0, 0.999) * 256}, ${this.clamp(this.b, 0.0, 0.999) * 256})`
    ctx.fillRect(x, y, 1, 1)
  }

  // clamp = (x, min, max) => x < min ? min : x > max ? : max : x
  clamp = (x, min, max) => {
    if (x < min) return min
    if (x > max) return max
    return x
  }

  add = (col) => new Color(this.r + col.r, this.g + col.g, this.b + col.b)
  mul = (val) => new Color(this.r * val, this.g * val, this.b * val)
  div = (val) => new Color(this.r / val, this.g / val, this.b / val)

  addThis = (col) => {
    this.r += col.r
    this.g += col.g
    this.b += col.b
  }
  // mul = (val) => new Color(this.r * val, this.g * val, this.b * val)
  mulThis = (val) => {
    this.r *= val
    this.g *= val
    this.b *= val
  }
  divThis = (val) => {
    this.r /= val
    this.g /= val
    this.b /= val
  }

}

class Ray {
  constructor(origin, direction) {
    this.origin = origin
    this.direction = direction
  }

  at = t => this.origin.add(this.direction.mul(t))

  rayColor = world => {
    let rec = new HitRecord()
    // if (world.hit(this, 0, Infinity, rec)) {
    let [hit, returnedRecord] = world.hit(this, 0, Infinity, rec)
    if (hit) {
      let color = new Color(returnedRecord.normal.x, returnedRecord.normal.y, returnedRecord.normal.z)
      return color.add(new Color(1, 1, 1)).mul(0.5)
    }
    let unitDirection = this.direction.unitVector()
    let t = 0.5 * unitDirection.y + 1.0
    return new Color(1, 1, 1).mul(1.0 - t).add(new Color(0.5, 0.7, 1.0).mul(t))
  }

  // rayColor = () => {
  //   let t = hitSphere(new Vec3(0, 0, -1), 0.5, this)
  //   if (t > 0.0) {
  //     let N = this.at(t).sub(new Vec3(0, 0, -1)).unitVector()
  //     return new Color(N.x + 1, N.y + 1, N.z + 1).mul(0.5)
  //   }
  //   let unitDirection = this.direction.unitVector()
  //   t = 0.5 * unitDirection.y + 1.0
  //   return new Color(1, 1, 1).mul(1.0 - t).add(new Color(0.5, 0.7, 1.0).mul(t))
  // }
}

const hitSphere = (center, radius, r) => {
  let oc = r.origin.sub(center)
  let a = r.direction.lengthSquared()
  let halfB = oc.dot(r.direction)
  let c = oc.lengthSquared() - radius * radius
  let discriminant = halfB * halfB - a * c
  return discriminant < 0 ? - 1.0 : (- halfB - Math.sqrt(discriminant)) / a
}

class HitRecord {
  constructor() {
    this.p = new Vec3(0, 0, 0)
    this.normal = new Vec3(0, 0, 0)
    this.t = 0
    this.frontFace = false
  }

  setFaceNormal = (r, outwardNormal) => {
    // if (r.direction.dot(outwardNormal) < 0) {
    //   this.frontFace = true
    // } else {
    //   this.frontFace = false
    // }
    // if (this.frontFace === true) {
    //   this.normal =
    // }
    this.frontFace = r.direction.dot(outwardNormal) < 0
    this.normal = this.frontFace ? outwardNormal : outwardNormal.mul(-1)
  }
}

class Sphere {
  constructor(center, radius) {
    this.center = center
    this.radius = radius
  }

  hit = (r, tMin, tMax, rec) => {
    let oc = r.origin.sub(this.center)
    let a = r.direction.lengthSquared()
    let halfB = oc.dot(r.direction)
    let c = oc.lengthSquared() - this.radius * this.radius
    let discriminant = halfB * halfB - a * c

    if (discriminant > 0) {
      let root = Math.sqrt(discriminant)
      let temp = (- halfB - root) / a
      if (temp < tMax && temp > tMin) {
        rec.t = temp
        rec.p = r.at(rec.t)
        let outwardNormal = (rec.p.sub(this.center).div(this.radius))
        rec.setFaceNormal(r, outwardNormal)
        return [true, rec]
      }
      temp = (-halfB + root) / a
      if (temp < tMax && temp > tMin) {
        rec.t = temp
        rec.p = r.at(rec.t)
        let outwardNormal = (rec.p.sub(this.center).div(this.radius))
        rec.setFaceNormal(r, outwardNormal)
        return [true, rec]
      }
    }
    return [false, rec]
  }
}

class HittableList {
  constructor() {
    this.objects = []
  }

  clear = () => this.objects = []
  add = object => this.objects.push(object)

  hit = (r, tMin, tMax, rec) => {
    // rayon, record vide au debut
    let tempRec = new HitRecord()
    let hitAnything = false
    let closestSoFar = tMax
    this.objects.forEach(object => {
      let [hitten, returnedRecord] = object.hit(r, tMin, closestSoFar, tempRec)
      if (hitten) {
        hitAnything = true
        closestSoFar = returnedRecord.t
        rec = returnedRecord
      }
      // if (object.hit(r, tMin, closestSoFar, tempRec)) {
      //   hitAnything = true
      //   closestSoFar = tempRec.t
      //   rec = tempRec
      // }
    })

    return [hitAnything, rec]
  }
}

const render = (ctx, canvas) => {
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  let camera = new Camera()
  let world = new HittableList()
  world.add(new Sphere(new Vec3(0, 0, -1), 0.5))
  world.add(new Sphere(new Vec3(0, -100.5, -1), 100))

  for (let j = IMAGE_HEIGHT - 1; j >= 0; j--) {
    // console.log(j)
    for (let i = 0; i < IMAGE_WIDTH; i++) {
      let pixelColor = new Color(0, 0, 0)
      for (let s = 0; s < SAMPLE_PER_PIXEL; s++) {
        let u = (i + Math.random()) / (IMAGE_WIDTH - 1)
        let v = (j + Math.random()) / (IMAGE_HEIGHT - 1)
        let r = camera.getRay(u, v)
        pixelColor.addThis(r.rayColor(world))
      }
      pixelColor.draw(ctx, DRAWING_OFFSET_X + i, (DRAWING_OFFSET_Y  + IMAGE_HEIGHT) - j)
    }
  }
}



const loop = () => {
  requestAnimationFrame(loop)
}

render(ctx, canvas)
loop()
