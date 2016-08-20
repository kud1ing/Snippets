# Convert a SVG file to a PDF file using Apache Batik

* download `batik-rasterizer-1.7.jar` from https://xmlgraphics.apache.org/batik/download.html
  * (1.8 has a bug: `NoClassDefFoundError: org/apache/batik/dom/svg/SVGDOMImplementation`)
* execute `java -jar batik-rasterizer.jar -m application/pdf <SVG_FILE>`
