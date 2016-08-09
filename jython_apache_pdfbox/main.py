"""
1. install `jython` (the standalone version can't import the external JARs)
2. get the external JARs
3. execute the script: `jython main.py` 
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
