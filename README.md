# webgame-raylib

```javascript
function collision(rect, circle){
  var NearestX = Max(rect.x, Min(circle.pos.x, rect.x + rect.w));
  var NearestY = Max(rect.y, Min(circle.pos.y, rect.y + rect.h));    
  var dist = createVector(circle.pos.x - NearestX, circle.pos.y - NearestY);

  if (circle.vel.dot(dist) < 0) { //if circle is moving toward the rect
    //update circle.vel using one of the above methods
  }

  var penetrationDepth = circle.r - dist.mag();
  var penetrationVector = dist.normalise().mult(penetrationDepth);
  circle.pos = circle.pos.sub(penetrationVector);
}
```# alone_in_dark
