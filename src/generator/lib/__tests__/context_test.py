import json
import unittest
from pprint import pprint

from generator.lib.util import new_context
from . import DictObject
from .test_data.discovery_document import DISCOVERY_DOC


class ContextTest(unittest.TestCase):
    def setUp(self):
        self.discovery_doc = json.loads(DISCOVERY_DOC)

    def test_sta_map(self):
        expected = DictObject({
            "AddEnrichmentToAlbumRequest": {
                "photoslibrary.albums.addEnrichment": ["request"]
            },
            "AddEnrichmentToAlbumResponse": {
                "photoslibrary.albums.addEnrichment": ["response"]
            },
            "Album": {
                "photoslibrary.albums.addEnrichment": [],
                "photoslibrary.albums.create": ["response"],
                "photoslibrary.albums.get": ["response"],
                "photoslibrary.albums.list": [],
                "photoslibrary.albums.share": [],
                "photoslibrary.sharedAlbums.get": ["response"],
            },
            "BatchCreateMediaItemsRequest": {
                "photoslibrary.mediaItems.batchCreate": ["request"]
            },
            "BatchCreateMediaItemsResponse": {
                "photoslibrary.mediaItems.batchCreate": ["response"]
            },
            "CreateAlbumRequest": {"photoslibrary.albums.create": ["request"]},
            "JoinSharedAlbumRequest": {"photoslibrary.sharedAlbums.join": ["request"]},
            "JoinSharedAlbumResponse": {
                "photoslibrary.sharedAlbums.join": ["response"]
            },
            "ListAlbumsResponse": {"photoslibrary.albums.list": ["response"]},
            "ListMediaItemsResponse": {"photoslibrary.mediaItems.list": ["response"]},
            "ListSharedAlbumsResponse": {
                "photoslibrary.sharedAlbums.list": ["response"]
            },
            "MediaItem": {
                "photoslibrary.mediaItems.batchCreate": [],
                "photoslibrary.mediaItems.get": ["response"],
                "photoslibrary.mediaItems.list": [],
                "photoslibrary.mediaItems.search": [],
            },
            "SearchMediaItemsRequest": {"photoslibrary.mediaItems.search": ["request"]},
            "SearchMediaItemsResponse": {
                "photoslibrary.mediaItems.search": ["response"]
            },
            "ShareAlbumRequest": {"photoslibrary.albums.share": ["request"]},
            "ShareAlbumResponse": {"photoslibrary.albums.share": ["response"]},
            "SharedAlbum": {
                "photoslibrary.sharedAlbums.get": [],
                "photoslibrary.sharedAlbums.join": [],
                "photoslibrary.sharedAlbums.list": [],
            },
        })

        schemas = DictObject(self.discovery_doc["schemas"])
        resources = DictObject(self.discovery_doc["resources"])

        actual = new_context(schemas, resources).sta_map
        self.assertEqual(actual, expected)

    def test_fqan_map(self):
        expected = DictObject({
            "photoslibrary.mediaItems.batchCreate": {
                "flatPath": "v1/mediaItems:batchCreate",
                "path": "v1/mediaItems:batchCreate",
                "id": "photoslibrary.mediaItems.batchCreate",
                "request": {"$ref": "BatchCreateMediaItemsRequest"},
                "description": "Creates one or more media items in a user's Google Photos library.\n\nThis is the second step for creating a media item. For details regarding\nStep 1, uploading the raw bytes to a Google Server, see\n<a href=\"/photos/library/guides/upload-media\">Uploading media</a>.\n\nThis call adds the media item to the library. If an album `id` is\nspecified, the call adds the media item to the album too. By default, the\nmedia item will be added to the end of the library or album.\n\nIf an album `id` and position are both defined, the media item is\nadded to the album at the specified position.\n\nIf the call contains multiple media items, they're added at the specified\nposition.\nIf you are creating a media item in a shared album where you are not the\nowner, you are not allowed to position the media item. Doing so will result\nin a `BAD REQUEST` error.",
                "response": {"$ref": "BatchCreateMediaItemsResponse"},
                "parameterOrder": [],
                "httpMethod": "POST",
                "scopes": [
                    "https://www.googleapis.com/auth/photoslibrary",
                    "https://www.googleapis.com/auth/photoslibrary.appendonly",
                    "https://www.googleapis.com/auth/photoslibrary.sharing",
                ],
                "parameters": {},
            },
            "photoslibrary.mediaItems.search": {
                "flatPath": "v1/mediaItems:search",
                "id": "photoslibrary.mediaItems.search",
                "path": "v1/mediaItems:search",
                "description": "Searches for media items in a user's Google Photos library.\nIf no filters are set, then all media items in the user's library are\nreturned.\nIf an album is set, all media items in the specified album are returned.\nIf filters are specified, media items that match the filters from the user's\nlibrary are listed.\nIf you set both the album and the filters, the request results in an error.",
                "request": {"$ref": "SearchMediaItemsRequest"},
                "httpMethod": "POST",
                "parameterOrder": [],
                "response": {"$ref": "SearchMediaItemsResponse"},
                "parameters": {},
                "scopes": [
                    "https://www.googleapis.com/auth/photoslibrary",
                    "https://www.googleapis.com/auth/photoslibrary.readonly",
                    "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata",
                ],
            },
            "photoslibrary.mediaItems.list": {
                "flatPath": "v1/mediaItems",
                "id": "photoslibrary.mediaItems.list",
                "path": "v1/mediaItems",
                "description": "List all media items from a user's Google Photos library.",
                "httpMethod": "GET",
                "response": {"$ref": "ListMediaItemsResponse"},
                "parameterOrder": [],
                "parameters": {
                    "pageToken": {
                        "location": "query",
                        "description": "A continuation token to get the next page of the results. Adding this to\nthe request returns the rows after the `pageToken`. The `pageToken` should\nbe the value returned in the `nextPageToken` parameter in the response to\nthe `listMediaItems` request.",
                        "type": "string",
                    },
                    "pageSize": {
                        "location": "query",
                        "description": "Maximum number of media items to return in the response. The default number\nof media items to return at a time is 25. The maximum `pageSize` is 100.",
                        "format": "int32",
                        "type": "integer",
                    },
                },
                "scopes": [
                    "https://www.googleapis.com/auth/photoslibrary",
                    "https://www.googleapis.com/auth/photoslibrary.readonly",
                    "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata",
                ],
            },
            "photoslibrary.mediaItems.get": {
                "description": "Returns the media item for the specified media item `id`.",
                "response": {"$ref": "MediaItem"},
                "parameterOrder": ["mediaItemId"],
                "httpMethod": "GET",
                "parameters": {
                    "mediaItemId": {
                        "description": "Identifier of media item to be requested.",
                        "required": True,
                        "type": "string",
                        "pattern": "^[^/]+$",
                        "location": "path",
                    }
                },
                "scopes": [
                    "https://www.googleapis.com/auth/photoslibrary",
                    "https://www.googleapis.com/auth/photoslibrary.readonly",
                    "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata",
                ],
                "flatPath": "v1/mediaItems/{mediaItemsId}",
                "path": "v1/mediaItems/{+mediaItemId}",
                "id": "photoslibrary.mediaItems.get",
            },
            "photoslibrary.sharedAlbums.get": {
                "httpMethod": "GET",
                "response": {"$ref": "Album"},
                "parameterOrder": ["shareToken"],
                "parameters": {
                    "albumId": {
                        "location": "query",
                        "description": "Identifier of the album to be requested. Must not be set if `shareToken` is\nset.",
                        "type": "string",
                    },
                    "shareToken": {
                        "location": "path",
                        "description": "Share token of the album to be request. Must not be set if `albumId` is\nset.",
                        "required": True,
                        "type": "string",
                        "pattern": "^[^/]+$",
                    },
                },
                "scopes": [
                    "https://www.googleapis.com/auth/photoslibrary",
                    "https://www.googleapis.com/auth/photoslibrary.readonly",
                    "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata",
                    "https://www.googleapis.com/auth/photoslibrary.sharing",
                ],
                "flatPath": "v1/sharedAlbums/{sharedAlbumsId}",
                "id": "photoslibrary.sharedAlbums.get",
                "path": "v1/sharedAlbums/{+shareToken}",
                "description": "Returns the album based on the specified `albumId` or `shareToken`.\nExactly one of `albumId` and `shareToken` must be set.\nThe `albumId` should be the ID of an album owned by the user or a shared\nalbum that the user has joined.",
            },
            "photoslibrary.sharedAlbums.list": {
                "response": {"$ref": "ListSharedAlbumsResponse"},
                "parameterOrder": [],
                "httpMethod": "GET",
                "scopes": [
                    "https://www.googleapis.com/auth/photoslibrary",
                    "https://www.googleapis.com/auth/photoslibrary.readonly",
                    "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata",
                ],
                "parameters": {
                    "pageSize": {
                        "description": "Maximum number of albums to return in the response. The default number of\nalbums to return at a time is 20. The maximum `pageSize` is 50.",
                        "format": "int32",
                        "type": "integer",
                        "location": "query",
                    },
                    "excludeNonAppCreatedData": {
                        "description": "If set, the results exclude media items that were not created by this app.\nDefaults to false (all albums are returned). This field is ignored if the\nphotoslibrary.readonly.appcreateddata scope is used.",
                        "type": "boolean",
                        "location": "query",
                    },
                    "pageToken": {
                        "location": "query",
                        "description": "A continuation token to get the next page of the results. Adding this to\nthe request returns the rows after the `pageToken`. The `pageToken` should\nbe the value returned in the `nextPageToken` parameter in the response to\nthe `listSharedAlbums` request.",
                        "type": "string",
                    },
                },
                "flatPath": "v1/sharedAlbums",
                "path": "v1/sharedAlbums",
                "id": "photoslibrary.sharedAlbums.list",
                "description": "Lists all shared albums available in the Sharing tab of the\nuser's Google Photos app.",
            },
            "photoslibrary.sharedAlbums.join": {
                "description": "Joins a shared album on behalf of the Google Photos user.",
                "request": {"$ref": "JoinSharedAlbumRequest"},
                "httpMethod": "POST",
                "parameterOrder": [],
                "response": {"$ref": "JoinSharedAlbumResponse"},
                "parameters": {},
                "scopes": ["https://www.googleapis.com/auth/photoslibrary.sharing"],
                "flatPath": "v1/sharedAlbums:join",
                "id": "photoslibrary.sharedAlbums.join",
                "path": "v1/sharedAlbums:join",
            },
            "photoslibrary.albums.list": {
                "flatPath": "v1/albums",
                "id": "photoslibrary.albums.list",
                "path": "v1/albums",
                "description": "Lists all albums shown to a user in the Albums tab of the Google\nPhotos app.",
                "httpMethod": "GET",
                "response": {"$ref": "ListAlbumsResponse"},
                "parameterOrder": [],
                "parameters": {
                    "pageToken": {
                        "location": "query",
                        "description": "A continuation token to get the next page of the results. Adding this to\nthe request returns the rows after the `pageToken`. The `pageToken` should\nbe the value returned in the `nextPageToken` parameter in the response to\nthe `listAlbums` request.",
                        "type": "string",
                    },
                    "pageSize": {
                        "location": "query",
                        "description": "Maximum number of albums to return in the response. The default number of\nalbums to return at a time is 20. The maximum `pageSize` is 50.",
                        "format": "int32",
                        "type": "integer",
                    },
                    "excludeNonAppCreatedData": {
                        "description": "If set, the results exclude media items that were not created by this app.\nDefaults to false (all albums are returned). This field is ignored if the\nphotoslibrary.readonly.appcreateddata scope is used.",
                        "type": "boolean",
                        "location": "query",
                    },
                },
                "scopes": [
                    "https://www.googleapis.com/auth/photoslibrary",
                    "https://www.googleapis.com/auth/photoslibrary.readonly",
                    "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata",
                ],
            },
            "photoslibrary.albums.get": {
                "flatPath": "v1/albums/{albumsId}",
                "id": "photoslibrary.albums.get",
                "path": "v1/albums/{+albumId}",
                "description": "Returns the album based on the specified `albumId` or `shareToken`.\nExactly one of `albumId` and `shareToken` must be set.\nThe `albumId` should be the ID of an album owned by the user or a shared\nalbum that the user has joined.",
                "httpMethod": "GET",
                "response": {"$ref": "Album"},
                "parameterOrder": ["albumId"],
                "parameters": {
                    "albumId": {
                        "location": "path",
                        "description": "Identifier of the album to be requested. Must not be set if `shareToken` is\nset.",
                        "required": True,
                        "type": "string",
                        "pattern": "^[^/]+$",
                    },
                    "shareToken": {
                        "location": "query",
                        "description": "Share token of the album to be request. Must not be set if `albumId` is\nset.",
                        "type": "string",
                    },
                },
                "scopes": [
                    "https://www.googleapis.com/auth/photoslibrary",
                    "https://www.googleapis.com/auth/photoslibrary.readonly",
                    "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata",
                    "https://www.googleapis.com/auth/photoslibrary.sharing",
                ],
            },
            "photoslibrary.albums.addEnrichment": {
                "response": {"$ref": "AddEnrichmentToAlbumResponse"},
                "parameterOrder": ["albumId"],
                "httpMethod": "POST",
                "scopes": [
                    "https://www.googleapis.com/auth/photoslibrary",
                    "https://www.googleapis.com/auth/photoslibrary.appendonly",
                    "https://www.googleapis.com/auth/photoslibrary.sharing",
                ],
                "parameters": {
                    "albumId": {
                        "description": "Identifier of the album where the enrichment is to be added.",
                        "required": True,
                        "type": "string",
                        "pattern": "^[^/]+$",
                        "location": "path",
                    }
                },
                "flatPath": "v1/albums/{albumsId}:addEnrichment",
                "path": "v1/albums/{+albumId}:addEnrichment",
                "id": "photoslibrary.albums.addEnrichment",
                "request": {"$ref": "AddEnrichmentToAlbumRequest"},
                "description": "Adds an enrichment at a specified position in a defined album.",
            },
            "photoslibrary.albums.create": {
                "response": {"$ref": "Album"},
                "parameterOrder": [],
                "httpMethod": "POST",
                "scopes": [
                    "https://www.googleapis.com/auth/photoslibrary",
                    "https://www.googleapis.com/auth/photoslibrary.appendonly",
                    "https://www.googleapis.com/auth/photoslibrary.sharing",
                ],
                "parameters": {},
                "flatPath": "v1/albums",
                "path": "v1/albums",
                "id": "photoslibrary.albums.create",
                "request": {"$ref": "CreateAlbumRequest"},
                "description": "Creates an album in a user's Google Photos library.",
            },
            "photoslibrary.albums.share": {
                "request": {"$ref": "ShareAlbumRequest"},
                "description": "Marks an album as shared and accessible to other users. This action can\nonly be performed on albums which were created by the developer via the\nAPI.",
                "response": {"$ref": "ShareAlbumResponse"},
                "parameterOrder": ["albumId"],
                "httpMethod": "POST",
                "scopes": ["https://www.googleapis.com/auth/photoslibrary.sharing"],
                "parameters": {
                    "albumId": {
                        "location": "path",
                        "description": "Identifier of the album to be shared. This `albumId` must belong to an\nalbum created by the developer.",
                        "required": True,
                        "type": "string",
                        "pattern": "^[^/]+$",
                    }
                },
                "flatPath": "v1/albums/{albumsId}:share",
                "path": "v1/albums/{+albumId}:share",
                "id": "photoslibrary.albums.share",
            },
        })

        schemas = DictObject(self.discovery_doc["schemas"])
        resources = DictObject(self.discovery_doc["resources"])

        actual = new_context(schemas, resources).fqan_map
        self.assertEqual(actual, expected)

    def test_rta_map(self):
        expected = DictObject({
            "mediaItems": ["batchCreate", "search", "list", "get"],
            "sharedAlbums": ["get", "list", "join"],
            "albums": ["list", "get", "addEnrichment", "create", "share"],
        })

        schemas = DictObject(self.discovery_doc["schemas"])
        resources = DictObject(self.discovery_doc["resources"])

        actual = new_context(schemas, resources).rta_map
        self.assertEqual(actual, expected)

    def test_rtc_map(self):
        expected = DictObject({
            "mediaItems": "photoslibrary",
            "sharedAlbums": "photoslibrary",
            "albums": "photoslibrary",
        })

        schemas = DictObject(self.discovery_doc["schemas"])
        resources = DictObject(self.discovery_doc["resources"])

        actual = new_context(schemas, resources).rtc_map
        self.assertEqual(actual, expected)

    def test_schemas(self):
        expected = DictObject({
            "EnrichmentItem": {
                "description": "An enrichment item.",
                "type": "object",
                "properties": {
                    "id": {
                        "description": "Identifier of the enrichment item.",
                        "type": "string",
                    }
                },
                "id": "EnrichmentItem",
                "used_by": ["AddEnrichmentToAlbumResponse"],
                "parents": [],
            },
            "BatchCreateMediaItemsRequest": {
                "description": "Request to create one or more media items in a user's Google Photos library.\nIf an `albumid` is specified, the media items are also added to that album.\n`albumPosition` is optional and can only be specified if an `albumId` is set.",
                "type": "object",
                "properties": {
                    "albumId": {
                        "description": "Identifier of the album where the media items are added. The media items\nare also added to the user's library. This is an optional field.",
                        "type": "string",
                    },
                    "newMediaItems": {
                        "description": "List of media items to be created.",
                        "type": "array",
                        "items": {
                            "$ref": "NewMediaItem",
                            "used_by": [],
                            "parents": ["BatchCreateMediaItemsRequest"],
                        },
                    },
                    "albumPosition": {
                        "$ref": "AlbumPosition",
                        "description": "Position in the album where the media items are added. If not\nspecified, the media items are added to the end of the album (as per\nthe default value, that is, `LAST_IN_ALBUM`). The request fails if this\nfield is set and the `albumId` is not specified. The request will also fail\nif you set the field and are not the owner of the shared album.",
                    },
                },
                "id": "BatchCreateMediaItemsRequest",
                "used_by": [],
                "parents": [],
            },
            "MapEnrichment": {
                "description": "An enrichment containing a map, showing origin and destination locations.",
                "type": "object",
                "properties": {
                    "destination": {
                        "$ref": "Location",
                        "description": "Destination location for this enrichemt item.",
                    },
                    "origin": {
                        "description": "Origin location for this enrichment item.",
                        "$ref": "Location",
                    },
                },
                "id": "MapEnrichment",
                "used_by": ["NewEnrichmentItem"],
                "parents": [],
            },
            "AddEnrichmentToAlbumRequest": {
                "description": "Request to add an enrichment to a specific album at a specific position.",
                "type": "object",
                "properties": {
                    "newEnrichmentItem": {
                        "$ref": "NewEnrichmentItem",
                        "description": "The enrichment to be added.",
                    },
                    "albumPosition": {
                        "description": "The position in the album where the enrichment is to be inserted.",
                        "$ref": "AlbumPosition",
                    },
                },
                "id": "AddEnrichmentToAlbumRequest",
                "used_by": [],
                "parents": [],
            },
            "DateRange": {
                "description": 'Defines a range of dates. Both dates must be of the same format. For more\ninformation, see <a href="#Date">Date</a>',
                "type": "object",
                "properties": {
                    "startDate": {
                        "description": "The start date (included as part of the range) in one of the formats\ndescribed.",
                        "$ref": "Date",
                    },
                    "endDate": {
                        "description": "The end date (included as part of the range). It must be specified in the\nsame format as the start date.",
                        "$ref": "Date",
                    },
                },
                "id": "DateRange",
                "used_by": ["DateFilter"],
                "parents": [],
            },
            "NewMediaItem": {
                "description": "New media item that's created in a user's Google Photos account.",
                "type": "object",
                "properties": {
                    "description": {
                        "description": "Description of the media item. This will be shown to the user in the item's\ninfo section in the Google Photos app.\nThis string shouldn't be more than 1000 characters.",
                        "type": "string",
                    },
                    "simpleMediaItem": {
                        "description": "A new media item that has been uploaded via the included `uploadToken`.",
                        "$ref": "SimpleMediaItem",
                    },
                },
                "id": "NewMediaItem",
                "used_by": ["BatchCreateMediaItemsRequest"],
                "parents": [],
            },
            "Status": {
                "description": "The `Status` type defines a logical error model that is suitable for different\nprogramming environments, including REST APIs and RPC APIs. It is used by\n[gRPC](https://github.com/grpc). The error model is designed to be:\n\n- Simple to use and understand for most users\n- Flexible enough to meet unexpected needs\n\n# Overview\n\nThe `Status` message contains three pieces of data: error code, error message,\nand error details. The error code should be an enum value of\ngoogle.rpc.Code, but it may accept additional error codes if needed.  The\nerror message should be a developer-facing English message that helps\ndevelopers *understand* and *resolve* the error. If a localized user-facing\nerror message is needed, put the localized message in the error details or\nlocalize it in the client. The optional error details may contain arbitrary\ninformation about the error. There is a predefined set of error detail types\nin the package `google.rpc` that can be used for common error conditions.\n\n# Language mapping\n\nThe `Status` message is the logical representation of the error model, but it\nis not necessarily the actual wire format. When the `Status` message is\nexposed in different client libraries and different wire protocols, it can be\nmapped differently. For example, it will likely be mapped to some exceptions\nin Java, but more likely mapped to some error codes in C.\n\n# Other uses\n\nThe error model and the `Status` message can be used in a variety of\nenvironments, either with or without APIs, to provide a\nconsistent developer experience across different environments.\n\nExample uses of this error model include:\n\n- Partial errors. If a service needs to return partial errors to the client,\n    it may embed the `Status` in the normal response to indicate the partial\n    errors.\n\n- Workflow errors. A typical workflow has multiple steps. Each step may\n    have a `Status` message for error reporting.\n\n- Batch operations. If a client uses batch request and batch response, the\n    `Status` message should be used directly inside batch response, one for\n    each error sub-response.\n\n- Asynchronous operations. If an API call embeds asynchronous operation\n    results in its response, the status of those operations should be\n    represented directly using the `Status` message.\n\n- Logging. If some API errors are stored in logs, the message `Status` could\n    be used directly after any stripping needed for security/privacy reasons.",
                "type": "object",
                "properties": {
                    "code": {
                        "description": "The status code, which should be an enum value of google.rpc.Code.",
                        "format": "int32",
                        "type": "integer",
                    },
                    "message": {
                        "description": "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client.",
                        "type": "string",
                    },
                    "details": {
                        "description": "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use.",
                        "type": "array",
                        "items": {
                            "type": "object",
                            "additionalProperties": {
                                "description": "Properties of the object. Contains field @type with type URL.",
                                "type": "any",
                                "used_by": [],
                                "parents": ["Status"],
                            },
                            "used_by": [],
                            "parents": ["Status"],
                        },
                    },
                },
                "id": "Status",
                "used_by": ["NewMediaItemResult"],
                "parents": [],
            },
            "MediaTypeFilter": {
                "description": "This filter defines the type of media items to be returned, for example,\nvideos or photos. All the specified media types are treated as an OR when\nused together.",
                "type": "object",
                "properties": {
                    "mediaTypes": {
                        "description": "The types of media items to be included. This field should be populated\nwith only one media type. If you specify multiple media types, it results\nin an error.",
                        "type": "array",
                        "items": {
                            "type": "string",
                            "enum": ["ALL_MEDIA", "VIDEO", "PHOTO"],
                            "used_by": [],
                            "parents": ["MediaTypeFilter"],
                        },
                        "enumDescriptions": [
                            "Treated as if no filters are applied. All media types are included.",
                            "All media items that are considered videos.\nThis also includes movies the user has created using the Google Photos\napp.",
                            "All media items that are considered photos. This includes .bmp, .gif,\n.ico, .jpg (and other spellings), .tiff, .webp and special photo types\nsuch as iOS live photos, Android motion photos, panoramas, photospheres.",
                        ],
                    }
                },
                "id": "MediaTypeFilter",
                "used_by": ["Filters"],
                "parents": [],
            },
            "SearchMediaItemsResponse": {
                "description": "List of media items that match the search parameters.",
                "type": "object",
                "properties": {
                    "nextPageToken": {
                        "description": "[Output only] Use this token to get the next set of media items. Its\npresence is the only reliable indicator of more media items being available\nin the next request.",
                        "type": "string",
                    },
                    "mediaItems": {
                        "description": "[Output only] List of media items that match the search parameters.",
                        "type": "array",
                        "items": {
                            "$ref": "MediaItem",
                            "used_by": [],
                            "parents": ["SearchMediaItemsResponse"],
                        },
                    },
                },
                "id": "SearchMediaItemsResponse",
                "used_by": [],
                "parents": [],
            },
            "LocationEnrichment": {
                "description": "An enrichment containing a single location.",
                "type": "object",
                "properties": {
                    "location": {
                        "$ref": "Location",
                        "description": "Location for this enrichment item.",
                    }
                },
                "id": "LocationEnrichment",
                "used_by": ["NewEnrichmentItem"],
                "parents": [],
            },
            "NewEnrichmentItem": {
                "description": "A new enrichment item to be added to an album, used by the\n`albums.addEnrichment` call.",
                "type": "object",
                "properties": {
                    "mapEnrichment": {
                        "$ref": "MapEnrichment",
                        "description": "Map to be added to the album.",
                    },
                    "textEnrichment": {
                        "$ref": "TextEnrichment",
                        "description": "Text to be added to the album.",
                    },
                    "locationEnrichment": {
                        "description": "Location to be added to the album.",
                        "$ref": "LocationEnrichment",
                    },
                },
                "id": "NewEnrichmentItem",
                "used_by": ["AddEnrichmentToAlbumRequest"],
                "parents": [],
            },
            "ListMediaItemsResponse": {
                "description": "List of all media items from the user's Google Photos library.",
                "type": "object",
                "properties": {
                    "nextPageToken": {
                        "description": "[Output only] Token to use to get the next set of media items. Its presence\nis the only reliable indicator of more media items being available in the\nnext request.",
                        "type": "string",
                    },
                    "mediaItems": {
                        "description": "[Output only] List of media items in the user's library.",
                        "type": "array",
                        "items": {
                            "$ref": "MediaItem",
                            "used_by": [],
                            "parents": ["ListMediaItemsResponse"],
                        },
                    },
                },
                "id": "ListMediaItemsResponse",
                "used_by": [],
                "parents": [],
            },
            "AlbumPosition": {
                "description": "Specifies a position in an album.",
                "type": "object",
                "properties": {
                    "relativeMediaItemId": {
                        "description": "The media item to which the position is relative to.\nOnly used when position type is AFTER_MEDIA_ITEM.",
                        "type": "string",
                    },
                    "position": {
                        "enumDescriptions": [
                            "Default value if this enum isn't set.",
                            "At the beginning of the album.",
                            "At the end of the album.",
                            "After a media item.",
                            "After an enrichment item.",
                        ],
                        "enum": [
                            "POSITION_TYPE_UNSPECIFIED",
                            "FIRST_IN_ALBUM",
                            "LAST_IN_ALBUM",
                            "AFTER_MEDIA_ITEM",
                            "AFTER_ENRICHMENT_ITEM",
                        ],
                        "description": "Type of position, for a media or enrichment item.",
                        "type": "string",
                    },
                    "relativeEnrichmentItemId": {
                        "description": "The enrichment item to which the position is relative to.\nOnly used when position type is AFTER_ENRICHMENT_ITEM.",
                        "type": "string",
                    },
                },
                "id": "AlbumPosition",
                "used_by": [
                    "BatchCreateMediaItemsRequest",
                    "AddEnrichmentToAlbumRequest",
                ],
                "parents": [],
            },
            "LatLng": {
                "description": 'An object representing a latitude/longitude pair. This is expressed as a pair\nof doubles representing degrees latitude and degrees longitude. Unless\nspecified otherwise, this must conform to the\n<a href="http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84\nstandard</a>. Values must be within normalized ranges.',
                "type": "object",
                "properties": {
                    "latitude": {
                        "description": "The latitude in degrees. It must be in the range [-90.0, +90.0].",
                        "format": "double",
                        "type": "number",
                    },
                    "longitude": {
                        "description": "The longitude in degrees. It must be in the range [-180.0, +180.0].",
                        "format": "double",
                        "type": "number",
                    },
                },
                "id": "LatLng",
                "used_by": ["Location"],
                "parents": [],
            },
            "BatchCreateMediaItemsResponse": {
                "description": "List of media items created.",
                "type": "object",
                "properties": {
                    "newMediaItemResults": {
                        "description": "[Output only] List of media items created.",
                        "type": "array",
                        "items": {
                            "$ref": "NewMediaItemResult",
                            "used_by": [],
                            "parents": ["BatchCreateMediaItemsResponse"],
                        },
                    }
                },
                "id": "BatchCreateMediaItemsResponse",
                "used_by": [],
                "parents": [],
            },
            "ShareInfo": {
                "description": "Information about albums that are shared. This information is only included\nif you created the album, it is shared and you have the sharing scope.",
                "type": "object",
                "properties": {
                    "isJoined": {
                        "description": "True if the user has joined the album. This is always true for the owner\nof the shared album.",
                        "type": "boolean",
                    },
                    "sharedAlbumOptions": {
                        "$ref": "SharedAlbumOptions",
                        "description": "Options that control the sharing of an album.",
                    },
                    "shareableUrl": {
                        "description": "A link to the album that's now shared on the Google Photos website and app.\nAnyone with the link can access this shared album and see all of the items\npresent in the album.",
                        "type": "string",
                    },
                    "shareToken": {
                        "description": "A token that can be used by other users to join this shared album via the\nAPI.",
                        "type": "string",
                    },
                },
                "id": "ShareInfo",
                "used_by": ["Album", "ShareAlbumResponse"],
                "parents": [],
            },
            "MediaItem": {
                "description": "Representation of a media item (such as a photo or video) in Google Photos.",
                "type": "object",
                "properties": {
                    "baseUrl": {
                        "description": "A URL to the media item's bytes. This shouldn't be used directly to access\nthe media item. For example, `'=w2048-h1024'` will set the dimensions of a\nmedia item of type photo to have a width of 2048 px and height of 1024 px.",
                        "type": "string",
                    },
                    "mimeType": {
                        "description": "MIME type of the media item. For example, `image/jpeg`.",
                        "type": "string",
                    },
                    "contributorInfo": {
                        "$ref": "ContributorInfo",
                        "description": "Information about the user who created this media item.",
                    },
                    "description": {
                        "description": "Description of the media item. This is shown to the user in the item's\ninfo section in the Google Photos app.",
                        "type": "string",
                    },
                    "mediaMetadata": {
                        "$ref": "MediaMetadata",
                        "description": "Metadata related to the media item, such as, height, width, or\ncreation time.",
                    },
                    "filename": {
                        "description": "Filename of the media item. This is shown to the user in the item's info\nsection in the Google Photos app.",
                        "type": "string",
                    },
                    "id": {
                        "description": "Identifier for the media item. This is a persistent identifier that can be\nused between sessions to identify this media item.",
                        "type": "string",
                    },
                    "productUrl": {
                        "description": "Google Photos URL for the media item. This link is available to\nthe user only if they're signed in.",
                        "type": "string",
                    },
                },
                "id": "MediaItem",
                "used_by": [
                    "SearchMediaItemsResponse",
                    "ListMediaItemsResponse",
                    "NewMediaItemResult",
                ],
                "parents": [],
            },
            "Album": {
                "description": "Representation of an album in Google Photos.\nAlbums are containers for media items. If an album has been shared by the\napplication, it contains an extra `shareInfo` property.",
                "type": "object",
                "properties": {
                    "mediaItemsCount": {
                        "description": "[Output only] The number of media items in the album.",
                        "format": "int64",
                        "type": "string",
                    },
                    "title": {
                        "description": "Name of the album displayed to the user in their Google Photos account.\nThis string shouldn't be more than 500 characters.",
                        "type": "string",
                    },
                    "coverPhotoBaseUrl": {
                        "description": "[Output only] A URL to the cover photo's bytes. This shouldn't be used as\nis. Parameters should be appended to this URL before use. For example,\n`'=w2048-h1024'` sets the dimensions of\nthe cover photo to have a width of 2048 px and height of 1024 px.",
                        "type": "string",
                    },
                    "isWriteable": {
                        "description": "[Output only] True if you can create media items in this album.\nThis field is based on the scopes granted and permissions of the album. If\nthe scopes are changed or permissions of the album are changed, this field\nis updated.",
                        "type": "boolean",
                    },
                    "coverPhotoMediaItemId": {
                        "description": "[Output only] Identifier for the media item associated with the cover\nphoto.",
                        "type": "string",
                    },
                    "id": {
                        "description": "[Ouput only] Identifier for the album. This is a persistent identifier that\ncan be used between sessions to identify this album.",
                        "type": "string",
                    },
                    "productUrl": {
                        "description": "[Output only] Google Photos URL for the album. The user needs to be signed\nin to their Google Photos account to access this link.",
                        "type": "string",
                    },
                    "shareInfo": {
                        "description": "[Output only] Information related to shared albums.\nThis field is only populated if the album is a shared album, the\ndeveloper created the album and the user has granted the\n`photoslibrary.sharing` scope.",
                        "$ref": "ShareInfo",
                    },
                },
                "id": "Album",
                "used_by": [
                    "ListAlbumsResponse",
                    "ListSharedAlbumsResponse",
                    "CreateAlbumRequest",
                    "JoinSharedAlbumResponse",
                ],
                "parents": [],
            },
            "JoinSharedAlbumRequest": {
                "description": "Request to join a shared album on behalf of the user. This uses a shareToken\nwhich can be acquired via the shareAlbum or listSharedAlbums calls.",
                "type": "object",
                "properties": {
                    "shareToken": {
                        "description": "Token to join the shared album on behalf of the user.",
                        "type": "string",
                    }
                },
                "id": "JoinSharedAlbumRequest",
                "used_by": [],
                "parents": [],
            },
            "SharedAlbumOptions": {
                "description": "Options that control the sharing of an album.",
                "type": "object",
                "properties": {
                    "isCollaborative": {
                        "description": "True if the shared album allows collaborators (users who have joined\nthe album) to add media items to it. Defaults to false.",
                        "type": "boolean",
                    },
                    "isCommentable": {
                        "description": "True if the shared album allows the owner and the collaborators (users\nwho have joined the album) to add comments to the album. Defaults to false.",
                        "type": "boolean",
                    },
                },
                "id": "SharedAlbumOptions",
                "used_by": ["ShareInfo", "ShareAlbumRequest"],
                "parents": [],
            },
            "TextEnrichment": {
                "description": "An enrichment containing text.",
                "type": "object",
                "properties": {
                    "text": {
                        "description": "Text for this enrichment item.",
                        "type": "string",
                    }
                },
                "id": "TextEnrichment",
                "used_by": ["NewEnrichmentItem"],
                "parents": [],
            },
            "ContentFilter": {
                "description": "This filter allows you to return media items based on the content type.\n\nIt's possible to specify a list of categories to include, and/or a list of\ncategories to exclude. Within each list, the categories are combined with an\nOR. <p>\nThe content filter `includedContentCategories`: [c1, c2, c3] would get media\nitems that contain (c1 OR c2 OR c3). <p>\nThe content filter `excludedContentCategories`: [c1, c2, c3] would NOT get\nmedia items that contain (c1 OR c2 OR c3). <p>\nYou can also include some categories while excluding others, as in this\nexample: `includedContentCategories`: [c1, c2], `excludedContentCategories`:\n[c3, c4] <p>\nThe previous example would get media items that contain (c1 OR c2) AND NOT\n(c3 OR c4). A category that appears in `includedContentategories` must not\nappear in `excludedContentCategories`.",
                "type": "object",
                "properties": {
                    "includedContentCategories": {
                        "enumDescriptions": [
                            "Default content category. This category is ignored when any other category\nis used in the filter.",
                            "Media items containing landscapes.",
                            "Media items containing receipts.",
                            "Media items containing cityscapes.",
                            "Media items containing landmarks.",
                            "Media items that are selfies.",
                            "Media items containing people.",
                            "Media items containing pets.",
                            "Media items from weddings.",
                            "Media items from birthdays.",
                            "Media items containing documents.",
                            "Media items taken during travel.",
                            "Media items containing animals.",
                            "Media items containing food.",
                            "Media items from sporting events.",
                            "Media items taken at night.",
                            "Media items from performances.",
                            "Media items containing whiteboards.",
                            "Media items that are screenshots.",
                            "Media items that are considered to be utility. These include, but aren't\nlimited to documents, screenshots, whiteboards etc.",
                        ],
                        "description": "The set of categories to be included in the media item search results.\nThe items in the set are ORed. There's a maximum of 10\n`includedContentCategories` per request.",
                        "type": "array",
                        "items": {
                            "type": "string",
                            "enum": [
                                "NONE",
                                "LANDSCAPES",
                                "RECEIPTS",
                                "CITYSCAPES",
                                "LANDMARKS",
                                "SELFIES",
                                "PEOPLE",
                                "PETS",
                                "WEDDINGS",
                                "BIRTHDAYS",
                                "DOCUMENTS",
                                "TRAVEL",
                                "ANIMALS",
                                "FOOD",
                                "SPORT",
                                "NIGHT",
                                "PERFORMANCES",
                                "WHITEBOARDS",
                                "SCREENSHOTS",
                                "UTILITY",
                            ],
                            "used_by": [],
                            "parents": ["ContentFilter"],
                        },
                    },
                    "excludedContentCategories": {
                        "enumDescriptions": [
                            "Default content category. This category is ignored when any other category\nis used in the filter.",
                            "Media items containing landscapes.",
                            "Media items containing receipts.",
                            "Media items containing cityscapes.",
                            "Media items containing landmarks.",
                            "Media items that are selfies.",
                            "Media items containing people.",
                            "Media items containing pets.",
                            "Media items from weddings.",
                            "Media items from birthdays.",
                            "Media items containing documents.",
                            "Media items taken during travel.",
                            "Media items containing animals.",
                            "Media items containing food.",
                            "Media items from sporting events.",
                            "Media items taken at night.",
                            "Media items from performances.",
                            "Media items containing whiteboards.",
                            "Media items that are screenshots.",
                            "Media items that are considered to be utility. These include, but aren't\nlimited to documents, screenshots, whiteboards etc.",
                        ],
                        "description": "The set of categories which are not to be included in the media item search\nresults. The items in the set are ORed. There's a maximum of 10\n`excludedContentCategories` per request.",
                        "type": "array",
                        "items": {
                            "enum": [
                                "NONE",
                                "LANDSCAPES",
                                "RECEIPTS",
                                "CITYSCAPES",
                                "LANDMARKS",
                                "SELFIES",
                                "PEOPLE",
                                "PETS",
                                "WEDDINGS",
                                "BIRTHDAYS",
                                "DOCUMENTS",
                                "TRAVEL",
                                "ANIMALS",
                                "FOOD",
                                "SPORT",
                                "NIGHT",
                                "PERFORMANCES",
                                "WHITEBOARDS",
                                "SCREENSHOTS",
                                "UTILITY",
                            ],
                            "type": "string",
                            "used_by": [],
                            "parents": ["ContentFilter"],
                        },
                    },
                },
                "id": "ContentFilter",
                "used_by": ["Filters"],
                "parents": [],
            },
            "ShareAlbumResponse": {
                "description": "Response to successfully sharing an album.",
                "type": "object",
                "properties": {
                    "shareInfo": {
                        "$ref": "ShareInfo",
                        "description": "[Output only] Information about the shared album.",
                    }
                },
                "id": "ShareAlbumResponse",
                "used_by": [],
                "parents": [],
            },
            "Date": {
                "description": "Represents a whole calendar date. The day may be 0 to represent a year and month where the day isn't significant, such as a whole calendar month. The month may be 0 to represent a a day and a year where the month isn't signficant, like when you want to specify the same day in every month of a year or a specific year. The year may be 0 to represent a month and day independent of year, like an anniversary date.",
                "type": "object",
                "properties": {
                    "year": {
                        "description": "Year of date. Must be from 1 to 9999, or 0 if specifying a date without\na year.",
                        "format": "int32",
                        "type": "integer",
                    },
                    "day": {
                        "description": "Day of month. Must be from 1 to 31 and valid for the year and month, or 0 if specifying a year/month where the day isn't significant.",
                        "format": "int32",
                        "type": "integer",
                    },
                    "month": {
                        "description": "Month of year. Must be from 1 to 12, or 0 if specifying a year without a\nmonth and day.",
                        "format": "int32",
                        "type": "integer",
                    },
                },
                "id": "Date",
                "used_by": ["DateRange", "DateFilter"],
                "parents": [],
            },
            "Filters": {
                "description": "Filters that can be applied to a media item search.\nIf multiple filter options are specified, they're treated as AND with each\nother.",
                "type": "object",
                "properties": {
                    "mediaTypeFilter": {
                        "description": "Filters the media items based on the type of media.",
                        "$ref": "MediaTypeFilter",
                    },
                    "excludeNonAppCreatedData": {
                        "description": "If set, the results exclude media items that were not created by this app.\nDefaults to false (all media items are returned). This field is ignored if\nthe photoslibrary.readonly.appcreateddata scope is used.",
                        "type": "boolean",
                    },
                    "dateFilter": {
                        "description": "Filters the media items based on their creation date.",
                        "$ref": "DateFilter",
                    },
                    "contentFilter": {
                        "description": "Filters the media items based on their content.",
                        "$ref": "ContentFilter",
                    },
                    "includeArchivedMedia": {
                        "description": "If set, the results include media items that the user has archived.\nDefaults to false (archived media items aren't included).",
                        "type": "boolean",
                    },
                },
                "id": "Filters",
                "used_by": ["SearchMediaItemsRequest"],
                "parents": [],
            },
            "MediaMetadata": {
                "description": "Metadata for a media item.",
                "type": "object",
                "properties": {
                    "video": {
                        "description": "Metadata for a video media type.",
                        "$ref": "Video",
                    },
                    "width": {
                        "description": "Original width (in pixels) of the media item.",
                        "format": "int64",
                        "type": "string",
                    },
                    "creationTime": {
                        "description": "Time when the media item was first created (not when it was uploaded to\nGoogle Photos).",
                        "format": "google-datetime",
                        "type": "string",
                    },
                    "height": {
                        "description": "Original height (in pixels) of the media item.",
                        "format": "int64",
                        "type": "string",
                    },
                    "photo": {
                        "description": "Metadata for a photo media type.",
                        "$ref": "Photo",
                    },
                },
                "id": "MediaMetadata",
                "used_by": ["MediaItem"],
                "parents": [],
            },
            "SearchMediaItemsRequest": {
                "description": "Request to search for media items in a user's library.\n\nIf the album id is specified, this call will return the list of media items\nin the album. If neither filters nor album id are\nspecified, this call will return all media items in a user's Google Photos\nlibrary.\n\nIf filters are specified, this call will return all media items in\nthe user's library that fulfill the filter criteria.\n\nFilters and album id must not both be set, as this will result in an\ninvalid request.",
                "type": "object",
                "properties": {
                    "albumId": {
                        "description": "Identifier of an album. If populated, lists all media items in\nspecified album. Can't set in conjunction with any filters.",
                        "type": "string",
                    },
                    "pageSize": {
                        "description": "Maximum number of media items to return in the response. The default number\nof media items to return at a time is 25. The maximum\n`pageSize` is 100.",
                        "format": "int32",
                        "type": "integer",
                    },
                    "filters": {
                        "$ref": "Filters",
                        "description": "Filters to apply to the request. Can't be set in conjunction with an\n`albumId`.",
                    },
                    "pageToken": {
                        "description": "A continuation token to get the next page of the results. Adding this to\nthe request returns the rows after the `pageToken`. The `pageToken` should\nbe the value returned in the `nextPageToken` parameter in the response to\nthe `searchMediaItems` request.",
                        "type": "string",
                    },
                },
                "id": "SearchMediaItemsRequest",
                "used_by": [],
                "parents": [],
            },
            "Location": {
                "description": "Represents a physical location.",
                "type": "object",
                "properties": {
                    "latlng": {
                        "description": "Position of the location on the map.",
                        "$ref": "LatLng",
                    },
                    "locationName": {
                        "description": "Name of the location to be displayed.",
                        "type": "string",
                    },
                },
                "id": "Location",
                "used_by": ["MapEnrichment", "LocationEnrichment"],
                "parents": [],
            },
            "Video": {
                "description": "Metadata that is specific to a video, for example, fps and processing status.\nSome of these fields may be null or not included.",
                "type": "object",
                "properties": {
                    "fps": {
                        "description": "Frame rate of the video.",
                        "format": "double",
                        "type": "number",
                    },
                    "cameraModel": {
                        "description": "Model of the camera with which the video was taken.",
                        "type": "string",
                    },
                    "status": {
                        "description": "Processing status of the video.",
                        "type": "string",
                        "enumDescriptions": [
                            "Video processing status is unknown.",
                            "Video is being processed. The user sees an icon for this\nvideo in the Google Photos app; however, it isn't playable yet.",
                            "Video processing is complete and it is now ready for viewing.",
                            "Something has gone wrong and the video has failed to process.",
                        ],
                        "enum": ["UNSPECIFIED", "PROCESSING", "READY", "FAILED"],
                    },
                    "cameraMake": {
                        "description": "Brand of the camera with which the video was taken.",
                        "type": "string",
                    },
                },
                "id": "Video",
                "used_by": ["MediaMetadata"],
                "parents": [],
            },
            "DateFilter": {
                "description": "This filter defines the allowed dates or date ranges for the media returned.\nIt's possible to pick a set of specific dates and a set of date ranges.",
                "type": "object",
                "properties": {
                    "dates": {
                        "description": "List of dates that match the media items' creation date. A maximum of\n5 dates can be included per request.",
                        "type": "array",
                        "items": {
                            "$ref": "Date",
                            "used_by": [],
                            "parents": ["DateFilter"],
                        },
                    },
                    "ranges": {
                        "description": "List of dates ranges that match the media items' creation date. A\nmaximum of 5 dates ranges can be included per request.",
                        "type": "array",
                        "items": {
                            "$ref": "DateRange",
                            "used_by": [],
                            "parents": ["DateFilter"],
                        },
                    },
                },
                "id": "DateFilter",
                "used_by": ["Filters"],
                "parents": [],
            },
            "NewMediaItemResult": {
                "description": "Result of creating a new media item.",
                "type": "object",
                "properties": {
                    "uploadToken": {
                        "description": "The upload token used to create this new media item.",
                        "type": "string",
                    },
                    "status": {
                        "$ref": "Status",
                        "description": 'If an error occurred during the creation of this media item, this field\nis  populated with information related to the error. For details regarding\nthis field, see <a href="#Status">Status</a>.',
                    },
                    "mediaItem": {
                        "$ref": "MediaItem",
                        "description": "Media item created with the upload token. It's populated if no errors\noccurred and the media item was created successfully.",
                    },
                },
                "id": "NewMediaItemResult",
                "used_by": ["BatchCreateMediaItemsResponse"],
                "parents": [],
            },
            "ListAlbumsResponse": {
                "description": "List of albums requested.",
                "type": "object",
                "properties": {
                    "nextPageToken": {
                        "description": "[Output only] Token to use to get the next set of albums. Populated if\nthere are more albums to retrieve for this request.",
                        "type": "string",
                    },
                    "albums": {
                        "description": "[Output only] List of albums shown in the Albums tab of the user's Google\nPhotos app.",
                        "type": "array",
                        "items": {
                            "$ref": "Album",
                            "used_by": [],
                            "parents": ["ListAlbumsResponse"],
                        },
                    },
                },
                "id": "ListAlbumsResponse",
                "used_by": [],
                "parents": [],
            },
            "ListSharedAlbumsResponse": {
                "description": "List of shared albums requested.",
                "type": "object",
                "properties": {
                    "nextPageToken": {
                        "description": "[Output only] Token to use to get the next set of shared albums. Populated\nif there are more shared albums to retrieve for this request.",
                        "type": "string",
                    },
                    "sharedAlbums": {
                        "description": "[Output only] List of shared albums.",
                        "type": "array",
                        "items": {
                            "$ref": "Album",
                            "used_by": [],
                            "parents": ["ListSharedAlbumsResponse"],
                        },
                    },
                },
                "id": "ListSharedAlbumsResponse",
                "used_by": [],
                "parents": [],
            },
            "CreateAlbumRequest": {
                "description": "Request to create an album in Google Photos.",
                "type": "object",
                "properties": {
                    "album": {
                        "description": "The album to be created.",
                        "$ref": "Album",
                    }
                },
                "id": "CreateAlbumRequest",
                "used_by": [],
                "parents": [],
            },
            "SimpleMediaItem": {
                "description": "A simple media item to be created in Google Photos via an upload token.",
                "type": "object",
                "properties": {
                    "uploadToken": {
                        "description": "Token identifying the media bytes that have been uploaded to Google.",
                        "type": "string",
                    }
                },
                "id": "SimpleMediaItem",
                "used_by": ["NewMediaItem"],
                "parents": [],
            },
            "AddEnrichmentToAlbumResponse": {
                "description": "The enrichment item that's created.",
                "type": "object",
                "properties": {
                    "enrichmentItem": {
                        "description": "[Output only] Enrichment which was added.",
                        "$ref": "EnrichmentItem",
                    }
                },
                "id": "AddEnrichmentToAlbumResponse",
                "used_by": [],
                "parents": [],
            },
            "ContributorInfo": {
                "description": "Information about the user who added the media item. Note that this\ninformation is included only if the media item is within a shared album\ncreated by your app and you have the sharing scope.",
                "type": "object",
                "properties": {
                    "profilePictureBaseUrl": {
                        "description": "URL to the profile picture of the contributor.",
                        "type": "string",
                    },
                    "displayName": {
                        "description": "Display name of the contributor.",
                        "type": "string",
                    },
                },
                "id": "ContributorInfo",
                "used_by": ["MediaItem"],
                "parents": [],
            },
            "JoinSharedAlbumResponse": {
                "description": "Response to successfully joining the shared album on behalf of the user.",
                "type": "object",
                "properties": {
                    "album": {
                        "$ref": "Album",
                        "description": "Shared album that the user has joined.",
                    }
                },
                "id": "JoinSharedAlbumResponse",
                "used_by": [],
                "parents": [],
            },
            "ShareAlbumRequest": {
                "description": "Request to make an album shared in Google Photos.",
                "type": "object",
                "properties": {
                    "sharedAlbumOptions": {
                        "description": "Options to be set when converting the album to a shared album.",
                        "$ref": "SharedAlbumOptions",
                    }
                },
                "id": "ShareAlbumRequest",
                "used_by": [],
                "parents": [],
            },
            "Photo": {
                "description": "Metadata that is specific to a photo, such as, ISO, focal length and\nexposure time. Some of these fields may be null or not included.",
                "type": "object",
                "properties": {
                    "cameraModel": {
                        "description": "Model of the camera with which the photo was taken.",
                        "type": "string",
                    },
                    "cameraMake": {
                        "description": "Brand of the camera with which the photo was taken.",
                        "type": "string",
                    },
                    "focalLength": {
                        "description": "Focal length of the camera lens with which the photo was taken.",
                        "format": "float",
                        "type": "number",
                    },
                    "isoEquivalent": {
                        "description": "ISO of the camera with which the photo was taken.",
                        "format": "int32",
                        "type": "integer",
                    },
                    "apertureFNumber": {
                        "description": "Aperture f number of the camera lens with which the photo was taken.",
                        "format": "float",
                        "type": "number",
                    },
                    "exposureTime": {
                        "description": "Exposure time of the camera aperture when the photo was taken.",
                        "format": "google-duration",
                        "type": "string",
                    },
                },
                "id": "Photo",
                "used_by": ["MediaMetadata"],
                "parents": [],
            },
        })

        schemas = DictObject(self.discovery_doc["schemas"])
        resources = DictObject(self.discovery_doc["resources"])

        actual = new_context(schemas, resources).schemas
        self.assertEqual(actual, expected)
