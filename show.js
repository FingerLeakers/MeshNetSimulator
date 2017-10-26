
function createShow (graph) {
  var self = {};

  function accessPath(o, path) {
    for (var i = 0; i < path.length; i++) {
      var key = path[i];
      if (key in o) {
        o = o[key];
      } else {
        return [];
      }
    }
    return o;
  }

  function createCallback(self, o, path) {
    return function() { self.showObject(o, path); }
  }

  self.showSelectedObject = function showSelectedObject (o) {
    var intNodes = graph.getSelectedIntNodes();
    var intLinks = graph.getSelectedIntLinks();

    if (intNodes.length == 1 && intLinks.length == 0) {
      self.showObject(intNodes[0].o, []);
    } else if (intNodes.length == 0 && intLinks.length == 1) {
      self.showObject(intLinks[0].o, []);
    } else {
      alert('Select only one node/link.');
    }
  }

  function setPathSelect(o, path) {
    var p = $('show_path');
    clearChildren(p);

    append(p, 'span', '/');
    for (var i = 0; i < path.length; i++) {
      append(p, 'span', path[i]).onclick = createCallback(self, o, path);
      append(p, 'span', '/');
    }
  }

  self.showObject = function showObject (o, path) {
    var tbody = document.getElementById('show_object_tbody');

    setPathSelect(o, path);
    $$('show_type').nodeValue = o.constructor.name;

    clearChildren(tbody);

    var obj = accessPath(o, path);

    for (var key in obj) {
      var value = obj[key];

      if (typeof value === 'function') {
        continue;
      }

      // Create table rows of properties
      var tr = append(tbody, 'tr');
      var td = append(tr, 'td', key);

      if (typeof value === 'string' || typeof value === 'number') {
        append(tr, 'td', value);
      } else if (Array.isArray(value)) {
        append(tr, 'td', '(' + value.length + ')');
        tr.onclick = createCallback(self, o, path.concat([key]));
      } else { // object
        append(tr, 'td', '(' + Object.keys(value).length + ')');
        tr.onclick = createCallback(self, o, path.concat([key]));
      } 
    }
  }

  return self;
}