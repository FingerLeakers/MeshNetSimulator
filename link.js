
function Link(id, p) {
  this.id = id;
  this.vpn = false;
  this.tq = 1.0;

  this.p = p;

  this.step = function () {
    console.log('step for link ' + this.id);
  }

  this.reset = function () {
  }
}
