const canvas = document.getElementById('canvas')
const ctx = canvas.getContext('2d')
ctx.canvas.width = window.innerWidth
ctx.canvas.height = window.innerHeight
ctx.font = '15px serif'


const DRAWING_OFFSET_X = 50
const DRAWING_OFFSET_Y = 50

const SAMPLE_PER_PIXEL = 100
const MAX_DEPTH = 50

const IMAGE_WIDTH = 400
const ASPECT_RATIO = 16.0 / 9.0
const IMAGE_HEIGHT = Math.floor(IMAGE_WIDTH / ASPECT_RATIO)


// Vector = [x, y, z]
const add = (vec1, vec2) => vec1.map((val, index) => val + vec2[index])
const sub = (vec1, vec2) => vec1.map((val, index) => val - vec2[index])
const mul = (vec1, value) => vec1.map((val, index) => val * value)
const div = (vec1, value) => vec1.map((val, index) => val / value)
const dot = (vec1, vec2) => vec1.reduce((acc, val, index) => acc + val * vec2[index], 0)
const lengthSquared = vec => vec.reduce((acc, val) => acc + val * val, 0)
const length = vec => Math.sqrt(lengthSquared(vec))
const unitVector = vec => div(vec, length(vec))
const at = (r, t) => add(r.origin, mul(r.direction, (t)))

const clamp = (x, min, max) => {
  if (x < min) return min
  if (x > max) return max
  return x
}

const drawPixel = (pixelColor, ctx, x, y) => {
  pixelColor = div(pixelColor, SAMPLE_PER_PIXEL)
  pixelColor = pixelColor.map(val => Math.sqrt(val))
  ctx.fillStyle = `rgb(${clamp(pixelColor[0], 0.0, 0.999) * 256}, ${clamp(pixelColor[1], 0.0, 0.999) * 256}, ${clamp(pixelColor[2], 0.0, 0.999) * 256})`
  ctx.fillRect(x, y, 1, 1)
}

const rayColor = (r, world, depth) => {
  let rec = new HitRecord()
  if (depth < 0) return [0, 0, 0]
  let [hit, returnedRecord] = world.hit(r, 0.001, Infinity, rec)
  if (hit) {
    let target = add(add(returnedRecord.p, returnedRecord.normal), randomInUnitVector())
    r.origin = returnedRecord.p
    r.direction = sub(target, returnedRecord.p)
    return mul(rayColor(r, world, depth - 1), 0.5)
    // let color = [returnedRecord.normal[0] + 1, returnedRecord.normal[1] + 1, returnedRecord.normal[2] + 1]
    // return mul(color, 0.5)
  }
  let unitDirection = unitVector(r.direction) // this.direction.unitVector()
  let t = 0.5 * unitDirection[1] + 1.0
  return add(mul([1, 1, 1], (1.0 - t)), mul([0.5, 0.7, 1.0], t))
}

const randomInUnitVector = () => {
  let a = Math.random() * 2 * Math.PI
  let z = - 1 + 2 * Math.random()
  let r = Math.sqrt(1 - z * z)
  return [Math.cos(a) * r, Math.sin(a) * r, z]
  // while (true) {
  //   let p = [- 1 + 2 * Math.random(), - 1 + 2 * Math.random(), - 1 + 2 * Math.random()]
  //   if (lengthSquared(p) < 1) return p
  // }
}

class Camera {
  constructor() {
    this.aspectRatio = 16.0 / 9.0
    this.viewportHeight = 2.0
    this.viewportWidth = this.aspectRatio * this.viewportHeight
    this.focalLength = 1.0

    this.origin = [0, 0, 0] // new Vec3(0, 0, 0)
    this.horizontal = [this.viewportWidth, 0, 0] // new Vec3(this.viewportWidth, 0, 0)
    this.vertical = [0, this.viewportHeight, 0] // new Vec3(0.0, this.viewportHeight, 0.0)
    // this.lowerLeftCorner = this.origin.sub(this.horizontal.div(2)).sub(this.vertical.div(2)).sub(new Vec3(0, 0, this.focalLength))
    this.lowerLeftCorner = sub(sub(sub(this.origin, div(this.horizontal, 2)), div(this.vertical, 2)), [0, 0, this.focalLength])
    console.log(this.lowerLeftCorner)
    let a = sub(this.origin, div(this.horizontal, 2))
    console.log(div(this.horizontal, 2))
  }
  getRay = (u, v) => ({origin: this.origin, direction: sub(add(add(this.lowerLeftCorner, mul(this.horizontal, u)), mul(this.vertical, v)), this.origin)})
}

class HitRecord {
  constructor() {
    this.p = [0, 0, 0] // new Vec3(0, 0, 0)
    this.normal = [0, 0, 0] // new Vec3(0, 0, 0)
    this.t = 0
    this.frontFace = false
  }

  setFaceNormal = (r, outwardNormal) => {
    this.frontFace = dot(r.direction, outwardNormal) < 0
    this.normal = this.frontFace ? outwardNormal : mul(outwardNormal, - 1)
  }
}

class Sphere {
  constructor(center, radius) {
    this.center = center
    this.radius = radius
  }

  hit = (r, tMin, tMax, rec) => {
    let oc = sub(r.origin, this.center)
    let a = lengthSquared(r.direction)
    let halfB = dot(oc, r.direction)
    let c = lengthSquared(oc) - this.radius * this.radius
    let discriminant = halfB * halfB - a * c

    if (discriminant > 0) {
      let root = Math.sqrt(discriminant)
      let temp = (- halfB - root) / a
      if (temp < tMax && temp > tMin) {
        rec.t = temp
        rec.p = at(r, rec.t)
        let outwardNormal = div(sub(rec.p, this.center), this.radius) // (rec.p.sub(this.center).div(this.radius))
        rec.setFaceNormal(r, outwardNormal)
        return true
      }
      temp = (- halfB + root) / a
      if (temp < tMax && temp > tMin) {
        rec.t = temp
        rec.p = at(r, rec.t)
        let outwardNormal = div(sub(rec.p, this.center), this.radius)
        rec.setFaceNormal(r, outwardNormal)
        return true
      }
    }
    return false
  }
}

class HittableList {
  constructor() {
    this.objects = []
  }

  clear = () => this.objects = []
  add = object => this.objects.push(object)

  hit = (r, tMin, tMax, rec) => {
    let tempRec = new HitRecord()
    let hitAnything = false
    let closestSoFar = tMax
    this.objects.forEach(object => {
      let hitten = object.hit(r, tMin, closestSoFar, tempRec)
      if (hitten) {
        hitAnything = true
        closestSoFar = tempRec.t
        rec = tempRec
      }
    })
    return [hitAnything, rec]
  }
}

const render = (ctx, canvas) => {
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  let camera = new Camera()
  let world = new HittableList()
  world.add(new Sphere([0, 0, -1], 0.5))
  world.add(new Sphere([0, - 100.5, - 1], 100))

  for (let j = IMAGE_HEIGHT - 1; j >= 0; j--) {
    for (let i = 0; i < IMAGE_WIDTH; i++) {
      let pixelColor = [0, 0, 0] // new Color(0, 0, 0)
      for (let s = 0; s < SAMPLE_PER_PIXEL; s++) {
        let u = (i + Math.random()) / (IMAGE_WIDTH - 1)
        let v = (j + Math.random()) / (IMAGE_HEIGHT - 1)
        let r = camera.getRay(u, v)
        pixelColor = add(pixelColor, rayColor(r, world, MAX_DEPTH))
      }
      drawPixel(pixelColor, ctx, DRAWING_OFFSET_X + i, (DRAWING_OFFSET_Y  + IMAGE_HEIGHT) - j)
    }
  }
}

const loop = () => {
  requestAnimationFrame(loop)
}

render(ctx, canvas)
loop()
