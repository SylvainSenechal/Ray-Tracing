const canvas = document.getElementById('canvas')
const ctx = canvas.getContext('2d')
ctx.canvas.width = window.innerWidth
ctx.canvas.height = window.innerHeight
ctx.font = '15px serif'


const DRAWING_OFFSET_X = 50
const DRAWING_OFFSET_Y = 50

const ASPECT_RATIO = 16.0 / 9.0
const IMAGE_WIDTH = 400
const IMAGE_HEIGHT = Math.floor(IMAGE_WIDTH / ASPECT_RATIO)
const VIEWPORT_HEIGHT = 2.0
const VIEWPORT_WIDTH = ASPECT_RATIO * VIEWPORT_HEIGHT
const FOCAL_LENGTH = 1.0


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

const ORIGIN = new Vec3(0, 0, 0)
const HORIZONTAL = new Vec3(VIEWPORT_WIDTH, 0, 0)
const VERTICAL = new Vec3(0, VIEWPORT_HEIGHT, 0)
const LOWER_LEFT_CORNER = ORIGIN.sub(HORIZONTAL.div(2)).sub(VERTICAL.div(2)).sub(new Vec3(0, 0, FOCAL_LENGTH))
console.log(LOWER_LEFT_CORNER)




class Color {
  constructor(r, g, b) {
    this.r = r
    this.g = g
    this.b = b
  }

  draw = (ctx, x, y) => {
    ctx.fillStyle = `rgb(${this.r * 255.99}, ${this.g * 255.99}, ${this.b * 255.99})`
    ctx.fillRect(x, y, 1, 1)
  }

  add = (vec) => new Color(this.r + vec.r, this.g + vec.g, this.b + vec.b)
  mul = (val) => new Color(this.r * val, this.g * val, this.b * val)

}

class Ray {
  constructor(origin, direction) {
    this.origin = origin
    this.direction = direction
  }

  at = t => this.origin.add(this.direction.mul(t))

  rayColor = () => {
    let t = hitSphere(new Vec3(0, 0, -1), 0.5, this)
    if (t > 0.0) {
      let N = this.at(t).sub(new Vec3(0, 0, -1)).unitVector()
      return new Color(N.x + 1, N.y + 1, N.z + 1).mul(0.5)
    }
    let unitDirection = this.direction.unitVector()
    t = 0.5 * unitDirection.y + 1.0
    return new Color(1, 1, 1).mul(1.0 - t).add(new Color(0.5, 0.7, 1.0).mul(t))
  }
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

  }

  setFaceNormal = (r, outwardNormal) => {
    this.frontFace = r.direction().dot(outwardNormal) < 0
    this.normal = this.frontFace ? outwardNormal : - outwardNormal
  }
}

class Sphere {
  constructor(center, radius) {
    this.center = center
    this.radius = radius
  }

  hit = (r, tMin, tMax) => {
    let oc = r.origin.sub(center)
    let a = r.direction.lengthSquared()
    let halfB = oc.dot(r.direction)
    let c = oc.lengthSquared() - radius * radius
    let discriminant = halfB * halfB - a * c

    if (discriminant > 0) {
      let root = Math.sqrt(discriminant)
      let temp = (- halfB - root) / a
      if (temp < tMax && temp > tMin) {
        rec.t = temp
        rec.p = r.at(rec.t)
        outwardNormal = (rec.p.sub(this.center).div(this.radius))
        rec.setFaceNormal(r, outwardNormal)
        return true
      }
      temp = (-halfB + root) / a
      if (temp < tMax && temp > tMin) {
        rec.t = temp
        rec.p = r.at(rec.t)
        outwardNormal = (rec.p.sub(this.center).div(this.radius))
        rec.setFaceNormal(r, outwardNormal)
        return true
      }
    }
    return false
  }
}


const render = (ctx, canvas) => {
  ctx.clearRect(0, 0, canvas.width, canvas.height)

  for (let j = IMAGE_HEIGHT - 1; j >= 0; j--) {
    // console.log(j)
    for (let i = 0; i < IMAGE_WIDTH; i++) {
      let u = i / (IMAGE_WIDTH - 1)
      let v = j / (IMAGE_HEIGHT - 1)
      let r = new Ray(ORIGIN, LOWER_LEFT_CORNER.add(HORIZONTAL.mul(u)).add(VERTICAL.mul(v)).sub(ORIGIN))
      // let pixelColor = new Color(i / (IMAGE_WIDTH - 1), j / (IMAGE_HEIGHT - 1), 0.25)
      let pixelColor = r.rayColor()//new Color(i / (IMAGE_WIDTH - 1), j / (IMAGE_HEIGHT - 1), 0.25)
      // console.log(pixelColor)
      pixelColor.draw(ctx, DRAWING_OFFSET_X + i, (DRAWING_OFFSET_Y  + IMAGE_HEIGHT) - j)
    }
  }
}



const loop = () => {
  requestAnimationFrame(loop)
}

render(ctx, canvas)
loop()
