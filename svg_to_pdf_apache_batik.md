# Convert a SVG file to a PDF file using Apache Batik

* download `batik-rasterizer-x.y.jar` from https://xmlgraphics.apache.org/batik/download.html
  * Version 1.8 had a bug which lead to `NoClassDefFoundError: org/apache/batik/dom/svg/SVGDOMImplementation`
* execute `java -jar batik-rasterizer-x.y.jar -m application/pdf <SVG_FILE>`
