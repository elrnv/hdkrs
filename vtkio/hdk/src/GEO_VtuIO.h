#pragma once

#include <GU/GU_Detail.h>
#include <GEO/GEO_IOTranslator.h>
#include <UT/UT_IStream.h>
#include <iostream>

class GEO_VtuIO : public GEO_IOTranslator
{
public:
	GEO_VtuIO() {}
	GEO_VtuIO(const GEO_VtuIO&) {}
	virtual ~GEO_VtuIO() {}
	virtual GEO_IOTranslator *duplicate() const;
	virtual const char *formatName() const;
	virtual int checkExtension(const char *);
	virtual int checkMagicNumber(unsigned);
	virtual GA_Detail::IOStatus fileLoad(GEO_Detail*, UT_IStream&, bool);
	virtual GA_Detail::IOStatus fileSave(const GEO_Detail*, std::ostream&);
};
