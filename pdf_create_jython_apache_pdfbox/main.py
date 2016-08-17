"""
1. install `jython` (the standalone JAR can't import the external JARs)
2. download the external JARs (see `EXTERNAL_JARS` below) from
  - https://pdfbox.apache.org/download.cgi
  - https://commons.apache.org/proper/commons-logging/download_logging.cgi
3. execute the current script: `jython main.py` 
"""

EXTERNAL_JARS = \
    [
        'commons-logging-1.2.jar',
        'pdfbox-2.0.2.jar',
        'fontbox-2.0.2.jar',
    ]

import sys
sys.path.extend(EXTERNAL_JARS)

from org.apache.pdfbox.pdmodel import PDDocument, PDPage, PDPageContentStream
from org.apache.pdfbox.pdmodel.font import PDType1Font

document = PDDocument()

page = PDPage()
document.addPage(page)

content = PDPageContentStream(document, page)

content.beginText()
content.setFont(PDType1Font.HELVETICA_BOLD, 12)
content.newLineAtOffset(100, 700)
content.drawString("Hello World")
content.endText()

content.close()

document.save("test.pdf")
document.close()
