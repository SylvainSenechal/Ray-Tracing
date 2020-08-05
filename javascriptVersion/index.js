const canvas = document.getElementById('canvas')
const ctx = canvas.getContext('2d')
ctx.canvas.width = window.innerWidth
ctx.canvas.height = window.innerHeight
ctx.font = '15px serif'


const DRAWING_OFFSET_X = 50
const DRAWING_OFFSET_Y = 50

const SAMPLE_PER_PIXEL = 500
const MAX_DEPTH = 50

const IMAGE_WIDTH = 400
const ASPECT_RATIO = 3.0 / 2.0
const IMAGE_HEIGHT = Math.floor(IMAGE_WIDTH / ASPECT_RATIO)


// Vector = [x, y, z]

// Classic version : 4~5 times faster than ES6 version
const add = (vec1, vec2) => [vec1[0] + vec2[0], vec1[1] + vec2[1], vec1[2] + vec2[2]]
const sub = (vec1, vec2) => [vec1[0] - vec2[0], vec1[1] - vec2[1], vec1[2] - vec2[2]]
const mul = (vec1, value) => [vec1[0] * value, vec1[1] * value, vec1[2] * value]
const div = (vec1, value) => [vec1[0] / value, vec1[1] / value, vec1[2] / value]
const dot = (vec1, vec2) => vec1[0] * vec2[0] + vec1[1] * vec2[1] + vec1[2] * vec2[2]
const mulVecVec = (vec1, vec2) => [vec1[0] * vec2[0], vec1[1] * vec2[1], vec1[2] * vec2[2]]
const lengthSquared = vec => vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2]
const cross = (vec1, vec2) => [
  vec1[1] * vec2[2] - vec1[2] * vec2[1],
  vec1[2] * vec2[0] - vec1[0] * vec2[2],
  vec1[0] * vec2[1] - vec1[1] * vec2[0],
]

// ES6 version
// const add = (vec1, vec2) => vec1.map((val, index) => val + vec2[index])
// const sub = (vec1, vec2) => vec1.map((val, index) => val - vec2[index])
// const mul = (vec1, value) => vec1.map((val, index) => val * value)
// const div = (vec1, value) => vec1.map((val, index) => val / value)
// const dot = (vec1, vec2) => vec1.reduce((acc, val, index) => acc + val * vec2[index], 0)
// const mulVecVec = (vec1, vec2) => vec1.map((val, index) => val * vec2[index])
// const lengthSquared = vec => vec.reduce((acc, val) => acc + val * val, 0)

const length = vec => Math.sqrt(lengthSquared(vec))
const unitVector = vec => div(vec, length(vec))



const at = (r, t) => add(r.origin, mul(r.direction, (t)))
const clamp = (x, min, max) => x < min ? min : x > max ? max : x
const reflect = (v, n) => sub(v, mul(n, 2 * dot(v, n)))
const refract = (uv, n, etaiOverEtat) => {
  let cosTheta = dot(mul(uv, - 1), n)
  let rOutPerp = mul(add(uv, mul(n, cosTheta)), etaiOverEtat)
  let rOutParallel = mul(n, - Math.sqrt(Math.abs(1.0 - lengthSquared(rOutPerp))))
  return add(rOutPerp, rOutParallel)
}
const schlick = (cosine, refIdx) => {
  let r0 = (1 - refIdx) / (1 + refIdx)
  r0 = r0 * r0
  return r0 + (1 - r0) * Math.pow(1 - cosine, 5)
}

const drawPixel = (pixelColor, ctx, x, y) => {
  pixelColor = div(pixelColor, SAMPLE_PER_PIXEL)
  pixelColor = pixelColor.map(val => Math.sqrt(val))
  ctx.fillStyle = `rgb(${clamp(pixelColor[0], 0.0, 0.999) * 256}, ${clamp(pixelColor[1], 0.0, 0.999) * 256}, ${clamp(pixelColor[2], 0.0, 0.999) * 256})`
  ctx.fillRect(x, y, 1, 1)
}

const scatterLambertian = (r, rec, mat) => {
  let scatterDirection = add(rec.normal, randomInUnitVector())
  let scattered = {origin: rec.p, direction: scatterDirection}
  let attenuation = mat.albedo
  return [true, scattered, attenuation]
}

const scatterMetal = (r, rec, mat) => {
  let reflected = reflect(unitVector(r.direction), rec.normal)
  let scattered = {origin: rec.p, direction: add(reflected, mul(randomInUnitSphere(), mat.fuzz))}
  let attenuation = mat.albedo
  return [dot(scattered.direction, rec.normal) > 0, scattered, attenuation]
}

const scatterDielectric = (r, rec, mat) => {
  let attenuation = [1.0, 1.0, 1.0]
  let etaiOverEtat = rec.frontFace ? (1.0 / mat.refIdx) : mat.refIdx

  let unitDirection = unitVector(r.direction)

  let cosTheta = Math.min(dot(mul(unitDirection, - 1), rec.normal), 1.0)
  let sinTheta = Math.sqrt(1.0 - cosTheta * cosTheta)
  if (etaiOverEtat * sinTheta > 1.0) {
    let reflected = reflect(unitDirection, rec.normal)
    let scattered = {origin: rec.p, direction: reflected}
      return [true, scattered, attenuation]
  }
  let reflectProb = schlick(cosTheta, etaiOverEtat)
  if (Math.random() < reflectProb) {
    let reflected = reflect(unitDirection, rec.normal)
    let scattered = {origin: rec.p, direction: reflected}
    return [true, scattered, attenuation]
  }

  let refracted = refract(unitDirection, rec.normal, etaiOverEtat)
  let scattered = {origin: rec.p, direction: refracted}

  return [true, scattered, attenuation]
}
let nbTotalRay = 0
const rayColor = (r, world, depth) => {
  nbTotalRay++
  let rec = new HitRecord()
  if (depth < 0) return [0, 0, 0]
  let [hit, returnedRecord] = world.hit(r, 0.001, Infinity, rec)

  if (hit) {
    if (returnedRecord.mat.type === 'lambertian') {
      let [scat, scatteredReturn, attenuationReturn] = scatterLambertian(r, returnedRecord, returnedRecord.mat)
      if (scat) {
        let col = rayColor(scatteredReturn, world, depth - 1)
        return mulVecVec(col, attenuationReturn)
      }
      return [0, 0, 0]
    }
    if (returnedRecord.mat.type === 'metal') {
      let [scat, scatteredReturn, attenuationReturn] = scatterMetal(r, returnedRecord, returnedRecord.mat)

      if (scat) {
        let col = rayColor(scatteredReturn, world, depth - 1)
        return mulVecVec(col, attenuationReturn)
      }
      return [0, 0, 0]
    }
    if (returnedRecord.mat.type === 'dielectric') {
      let [scat, scatteredReturn, attenuationReturn] = scatterDielectric(r, returnedRecord, returnedRecord.mat)

      if (scat) {
        let col = rayColor(scatteredReturn, world, depth - 1)
        return mulVecVec(col, attenuationReturn)
      }
      return [0, 0, 0]
    }




    // let target = add(add(returnedRecord.p, returnedRecord.normal), randomInUnitVector())
    // r.origin = returnedRecord.p
    // r.direction = sub(target, returnedRecord.p)
    // return mul(rayColor(r, world, depth - 1), 0.5)

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
}

const randomInUnitSphere = () => {
  while (true) {
    let p = [- 1 + 2 * Math.random(), - 1 + 2 * Math.random(), - 1 + 2 * Math.random()]
    if (lengthSquared(p) < 1) return p
  }
}

const randomInUnitDisk = () => {
  while (true) {
    let p = [- 1 + 2 * Math.random(), - 1 + 2 * Math.random(), 0]
    if (lengthSquared(p) < 1) return p
  }
}

class Camera {
  constructor(lookFrom, lookAt, vup, vFov, aspectRatio, aperture, focusDist) {
    let theta = vFov  * Math.PI / 180
    let h = Math.tan(theta / 2)
    let viewportHeight = 2.0 * h
    let viewportWidth = aspectRatio * viewportHeight
    let focalLength = 1.0

    this.w = unitVector(sub(lookFrom, lookAt))
    this.u = unitVector(cross(vup, this.w))
    this.v = cross(this.w, this.u)

    // this.aspectRatio = 16.0 / 9.0
    // this.viewportHeight = 2.0
    // this.viewportWidth = this.aspectRatio * this.viewportHeight
    // this.focalLength = 1.0

    this.origin = lookFrom // [0, 0, 0] // new Vec3(0, 0, 0)
    this.horizontal = mul(this.u, focusDist * viewportWidth) // [viewportWidth, 0, 0] // new Vec3(this.viewportWidth, 0, 0)
    this.vertical = mul(this.v, focusDist * viewportHeight) // [0, viewportHeight, 0] // new Vec3(0.0, this.viewportHeight, 0.0)
    // this.lowerLeftCorner = sub(sub(sub(this.origin, div(this.horizontal, 2)), div(this.vertical, 2)), [0, 0, focalLength])
    this.lowerLeftCorner = sub(sub(sub(this.origin, div(this.horizontal, 2)), div(this.vertical, 2)), mul(this.w, focusDist))
    this.lensRadius = aperture / 2
    console.log('ok')
    console.log(focusDist)
    console.log(this.origin)
    console.log(this.horizontal)
    console.log(this.vertical)
    console.log(this.lowerLeftCorner)
    console.log(this.lensRadius)
    console.log('oui')
  }
  // getRay = (u, v) => ({origin: this.origin, direction: sub(add(add(this.lowerLeftCorner, mul(this.horizontal, u)), mul(this.vertical, v)), this.origin)})
  // getRay = (u, v) => ({origin: this.origin, direction: sub(add(add(this.lowerLeftCorner, mul(this.horizontal, u)), mul(this.vertical, v)), this.origin)})
  getRay = (u, v) => {
    let rd = mul(randomInUnitDisk(), this.lensRadius)
    let offset = add(mul(this.u, rd[0]), mul(this.v, rd[1]))
    return {origin: add(this.origin, offset), direction: sub(sub(add(add(this.lowerLeftCorner, mul(this.horizontal, u)), mul(this.vertical, v)), this.origin), offset)}
  }
}

class HitRecord {
  constructor() {
    this.p = [0, 0, 0] // new Vec3(0, 0, 0)
    this.normal = [0, 0, 0] // new Vec3(0, 0, 0)
    this.t = 0
    this.frontFace = false
    this.mat = {albedo: [0, 0, 0], type:'lambertian'}
  }

  setFaceNormal = (r, outwardNormal) => {
    this.frontFace = dot(r.direction, outwardNormal) < 0
    this.normal = this.frontFace ? outwardNormal : mul(outwardNormal, - 1)
  }
}

class Sphere {
  constructor(center, radius, material) {
    this.center = center
    this.radius = radius
    this.material = material
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
        rec.mat = this.material
        return true
      }
      temp = (- halfB + root) / a
      if (temp < tMax && temp > tMin) {
        rec.t = temp
        rec.p = at(r, rec.t)
        let outwardNormal = div(sub(rec.p, this.center), this.radius)
        rec.setFaceNormal(r, outwardNormal)
        rec.mat = this.material
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

const randomScene = () => {
  let world = new HittableList()
  let groundMaterial = {type: 'lambertian', albedo: [0.5, 0.5, 0.5]}
  world.add(new Sphere([0, - 1000, 0], 1000, groundMaterial))

  for (let a = - 11; a < 11; a++) {
    for (let b = - 11; b < 11; b++) {
      let chooseMat = Math.random()
      let center = [
        a + 0.9 * Math.random(),
        0.2,
        b + 0.9 * Math.random()
      ]

      if (length(sub(center, [4, 0.2, 0])) > 0.9) {
        if (chooseMat < 0.8) {
          let albedo = mul([Math.random(), Math.random(), Math.random()], Math.random())
          let sphereMaterial = {type: 'lambertian', albedo: albedo}
          world.add(new Sphere(center, 0.2, sphereMaterial))
        } else if (chooseMat < 0.95) {
          let albedo = [0.5 + 0.5 * Math.random(), 0.5 + 0.5 * Math.random(), 0.5 + 0.5 * Math.random()]
          let fuzz = 0.5 * Math.random()
          let sphereMaterial = {type: 'metal', albedo: albedo, fuzz: fuzz}
          world.add(new Sphere(center, 0.2, sphereMaterial))
        } else {
          let sphereMaterial = {type: 'dielectric', refIdx: 1.5}
          world.add(new Sphere(center, 0.2, sphereMaterial))
        }
      }
    }
  }
  let mat1 = {type: 'dielectric', refIdx: 1.5}
  let mat2 = {type: 'lambertian', albedo: [0.4, 0.2, 0.1]}
  let mat3 = {type: 'metal', albedo: [0.7, 0.6, 0.5], fuzz: 0.0}
  world.add(new Sphere([0, 1, 0], 1.0, mat1))
  world.add(new Sphere([- 4, 1, 0], 1.0, mat2))
  world.add(new Sphere([4, 1, 0], 1.0, mat3))

  return world
}

const render = (ctx, canvas) => {
  ctx.clearRect(0, 0, canvas.width, canvas.height)

  // let lookFrom = [3, 3, 2]
  // let lookAt = [0, 0, - 1]
  // let vup = [0, 1, 0]
  // let distToFocus = length(sub(lookFrom, lookAt))
  // let aperture = 2.0
  // let camera = new Camera(lookFrom, lookAt, vup, 20, ASPECT_RATIO, aperture, distToFocus)

  // let world = new HittableList()
  // world.add(new Sphere([0, - 100.5, - 1], 100, {albedo: [0.8, 0.8, 0.0], type:'lambertian'}))
  // world.add(new Sphere([0, 0, -1], 0.5, {albedo: [0.7, 0.3, 0.3], type:'lambertian'}))
  // world.add(new Sphere([-1, 0, -1], 0.5, {albedo: [0.8, 0.8, 0.8], type:'metal', fuzz: 0.3}))
  // world.add(new Sphere([1, 0, -1], 0.5, {albedo: [0.8, 0.6, 0.2], type:'metal', fuzz: 1.0}))

  // world.add(new Sphere([0, - 100.5, - 1], 100, {albedo: [0.8, 0.8, 0.0], type:'lambertian'}))
  // world.add(new Sphere([0, 0, -1], 0.5, {type:'dielectric', refIdx: 1.5}))
  // world.add(new Sphere([-1, 0, -1], 0.5, {type:'dielectric', refIdx: 1.5}))
  // world.add(new Sphere([1, 0, -1], 0.5, {albedo: [0.8, 0.6, 0.2], type:'metal', fuzz: 1.0}))

  // world.add(new Sphere([0, - 100.5, - 1], 100, {albedo: [0.8, 0.8, 0.0], type:'lambertian'}))
  // world.add(new Sphere([0, 0, -1], 0.5, {albedo: [0.1, 0.2, 0.5], type:'lambertian'}))
  // world.add(new Sphere([-1, 0, -1], 0.5, {type:'dielectric', refIdx: 1.5}))
  // world.add(new Sphere([1, 0, -1], 0.5, {albedo: [0.8, 0.6, 0.2], type:'metal', fuzz: 0.0}))

// TODO: attention a ref idx > 1
  // world.add(new Sphere([0, - 100.5, - 1], 100, {albedo: [0.8, 0.8, 0.0], type:'lambertian'}))
  // world.add(new Sphere([0, 0, -1], 0.5, {albedo: [0.1, 0.2, 0.5], type:'lambertian'}))
  // world.add(new Sphere([-1, 0, -1], 0.5, {type:'dielectric', refIdx: 1.5}))
  // world.add(new Sphere([-1, 0, -1], -0.4, {type:'dielectric', refIdx: 1.5}))
  // world.add(new Sphere([1, 0, -1], 0.5, {albedo: [0.8, 0.6, 0.2], type:'metal', fuzz: 0.0}))

  // let R = Math.cos(Math.PI / 4)
  // world.add(new Sphere([- R, 0, - 1], R, {albedo: [0.0, 0.0, 1.0], type:'lambertian'}))
  // world.add(new Sphere([R, 0, - 1], R, {albedo: [1.0, 0.0, 0.0], type:'lambertian'}))

  // // TODO: attention a ref idx > 1
  //   world.add(new Sphere([0, - 100.5, - 1], 100, {albedo: [0.8, 0.8, 0.0], type:'lambertian'}))
  //   world.add(new Sphere([0, 0, -1], 0.5, {albedo: [0.1, 0.2, 0.5], type:'lambertian'}))
  //   world.add(new Sphere([-1, 0, -1], 0.5, {type:'dielectric', refIdx: 1.5}))
  //   world.add(new Sphere([-1, 0, -1], -0.45, {type:'dielectric', refIdx: 1.5}))
  //   world.add(new Sphere([1, 0, -1], 0.5, {albedo: [0.8, 0.6, 0.2], type:'metal', fuzz: 0.0}))


  let world = randomScene()

  let lookFrom = [13, 2, 3]
  let lookAt = [0, 0, 0]
  let vup = [0, 1, 0]
  let distToFocus = 10
  let aperture = 0.1
  let camera = new Camera(lookFrom, lookAt, vup, 20, ASPECT_RATIO, aperture, distToFocus)

  for (let j = IMAGE_HEIGHT - 1; j >= 0; j--) {
    console.log(`${nbTotalRay} rays traced`)
    console.log(`${IMAGE_HEIGHT - j} lines draw out of ${IMAGE_HEIGHT}`)
    for (let i = 0; i < IMAGE_WIDTH; i++) {
      let pixelColor = [0, 0, 0]
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
