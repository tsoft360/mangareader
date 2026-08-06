#! /bin/python
import sqlite3
from pathlib import Path
import shutil
import json

SOURCE = Path("to_parse")
DEST = Path("/home/dragon/.koma/Library")
DB = Path("komikku.db")

def format_chapter(number):
    if "." in number:
        return "Ch" + number.replace(".", "_")
    return f"Ch{int(number):03}"

def get_chapters(manga_id):
    db = sqlite3.connect(DB)
    cur = db.cursor()

    cur.execute("""
        SELECT slug, num, title
        FROM chapters
        WHERE manga_id=?
        AND downloaded=1
        ORDER BY num
    """, (manga_id,))

    chapters = cur.fetchall()
    db.close()

    return chapters

def write_chapter_metadata(path, slug, number, title):
    metadata = {
        "komikku_id": slug,
        "number": number,
        "title": title
    }

    with open(path / "chapter.json", "w", encoding="utf-8") as f:
        json.dump(
            metadata,
            f,
            indent=4,
            ensure_ascii=False
        )

def import_manga(manga_id, manga_name):
    chapters = get_chapters(manga_id)
    source_folder = SOURCE / manga_name
    dest_folder = DEST / manga_name

    dest_folder.mkdir(
        parents=True,
        exist_ok=True
    )

    for slug, number, title in chapters:
        old_path = source_folder / slug

        if not old_path.exists():
            print("Missing: ", slug)

        chapter_name = format_chapter(number)

        new_path = dest_folder / chapter_name

        print(
            f"{old_path.name} -> {new_path}"
        )

        shutil.copytree(
            old_path,
            new_path
        )

        write_chapter_metadata(
            new_path,
            slug,
            number,
            title
        )

if __name__ == '__main__':
    manga_id = 1

    import_manga(
        manga_id,
        "Bloom into You"
    )