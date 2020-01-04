var assert = require('assert');
var eleventy = require('./');

assert.equal(110, eleventy(), 'Eleventy should give you eleventy');
