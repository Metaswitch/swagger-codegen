/**
 * Swagger Petstore
 * This is a sample server Petstore server.  You can find out more about Swagger at [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/).  For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * OpenAPI spec version: 1.0.0
 * Contact: apiteam@swagger.io
 *
 * NOTE: This class is auto generated by the swagger code generator program.
 * https://github.com/swagger-api/swagger-codegen.git
 * Do not edit the class manually.
 */

/*
 * SWGTag.h
 * 
 * A tag for a pet
 */

#ifndef SWGTag_H_
#define SWGTag_H_

#include <QJsonObject>


#include <QString>

#include "SWGObject.h"

namespace Swagger {

class SWGTag: public SWGObject {
public:
    SWGTag();
    SWGTag(QString* json);
    virtual ~SWGTag();
    void init();
    void cleanup();

    QString asJson ();
    QJsonObject* asJsonObject();
    void fromJsonObject(QJsonObject &json);
    SWGTag* fromJson(QString &jsonString);

    qint64 getId();
    void setId(qint64 id);

    QString* getName();
    void setName(QString* name);


    virtual bool isSet() override;

private:
    qint64 id;
    bool m_id_isSet;
    
    QString* name;
    bool m_name_isSet;
    
};

}

#endif /* SWGTag_H_ */
