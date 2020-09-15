#!/bin/bash

fd -e md -x sd "(\[)([^\s-])" '[ $2' {} 
fd -e md -x sd "([^\s-])(\])" '$1 ]' {} 
