# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "google-api-python-client",
# ]
# ///
import json
import os
from pathlib import Path

import googleapiclient.discovery
from google.oauth2 import service_account

#   Defining the scope of the authorization request
SCOPES = ["https://www.googleapis.com/auth/androidpublisher"]

#   Package name for app
package_name = "de.fragdenstaat.scanner"


def upload_aab(aab_path: Path, track: str = "internal", status="completed"):
    #   Create a credentials object and create a service object using the credentials object
    service_account_info = json.loads(os.environ["GOOGLE_SERVICE_ACCOUNT"])
    credentials = service_account.Credentials.from_service_account_info(
        service_account_info, scopes=SCOPES
    )
    service = googleapiclient.discovery.build(
        "androidpublisher", "v3", credentials=credentials, cache_discovery=False
    )
    print("Sending edit request...")
    #   Create an edit request using the service object and get the editId
    edit_request = service.edits().insert(body={}, packageName=package_name)
    result = edit_request.execute()
    edit_id = result["id"]

    print("Uploading bundle...")
    #   Create a request to upload the app bundle
    try:
        bundle_response = (
            service.edits()
            .bundles()
            .upload(
                editId=edit_id,
                packageName=package_name,
                media_body=str(aab_path),
                media_mime_type="application/octet-stream",
            )
            .execute()
        )
    except Exception as err:
        print(f"There was an error while uploading a new version of {package_name}")
        raise err

    print(f"Version code {bundle_response['versionCode']} has been uploaded")

    #   Create a track request to upload the bundle to the beta track
    track_response = (
        service.edits()
        .tracks()
        .update(
            editId=edit_id,
            track=track,
            packageName=package_name,
            body={
                "releases": [
                    {
                        "versionCodes": [str(bundle_response["versionCode"])],
                        "status": status,
                    }
                ]
            },
        )
        .execute()
    )

    print(track_response)

    #   Create a commit request to commit the edit to track
    commit_request = (
        service.edits().commit(editId=edit_id, packageName=package_name).execute()
    )

    print(f"Edit {commit_request['id']} has been committed to track {track}")
    print(
        f"Version code {bundle_response['versionCode']} has been uploaded.\nEdit {commit_request['id']} has been committed"
    )


if __name__ == "__main__":
    aab_path = (
        Path(__file__).parent.parent
        / "src-tauri/gen/android/app/build/outputs/bundle/universalRelease/app-universal-release.aab"
    )
    upload_aab(aab_path, status="draft")
