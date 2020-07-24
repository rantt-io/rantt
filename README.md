# rantt

![Rust](https://github.com/rantt-io/rantt/workflows/Rust/badge.svg)

`rantt` is a project planning app written in Rust

## Tasks

Try running `rantt task`

```
% rantt task
No rantt config found. Create with `rant config init`
```

Initialize config file with the `config init` sub-commands

```
% rantt config init
Initialized config at ~/.rantt
```

Try running `rantt task` again.

```
% rantt task
0 tasks
```

We don't have any tasks yet. Let's create some.
Create a new task with the `task add` sub-commands.

```
% rantt task add High-level design doc
Created task #1

% rantt task add Security review
Created task #2
```

Now if we run `rantt task` again we'll see our tasks.

```
% rantt task

+----+-----------------------+--------+
| ID | Description           | Effort |
+----+-----------------------+--------+
| 1  | High-level design doc | 1d     |
+----+-----------------------+--------+
| 2  | Security review       | 1d     |
+----+-----------------------+--------+
```

Since we didn't specify a value for effort it was defaulted to `1d`. You can specify an effort at task creation time with `-e`.

```
% rantt task add -e 4d Update architecture doc
Created task #3

% rantt

+----+-------------------------+--------+
| ID | Description             | Effort |
+----+-------------------------+--------+
| 1  | High-level design doc   | 1d     |
+----+-------------------------+--------+
| 2  | Security review         | 1d     |
+----+-------------------------+--------+
| 3  | Update architecture doc | 4d     |
+----+-------------------------+--------+
```

In addition to effort, you can specify resources, projects, and start-before, start-after, need-by, and dependencies.

```
% rantt -d 1d -r lebaron -p DVR -n 2020/08/28 -d 1,2 Add cache key to interface model
Errors:
 - resource 'lebaron' doesn't exist
 - project 'DVR' doesn't exist
```

Oops, we tried to assign the task to a project and resource that don't don't exist. Create them with the `project` and `resource` sub-commands.

```
% rantt project create DVR
Created project 'DVR'

% rantt resource create lebaron
Created resoure 'lebaron'
```

Now, let's try to add the task again.

```
% rantt -d 1d -r lebaron -p DVR -n 2020/08/28 -d 1,2 Add cache key to interface model
Created task #4

% rantt task
+----+----------------------------------+--------+---------+----------+----------+--------------+
| ID | Description                      | Effort | Need-By |Resources | Projects | Dependencies |
+----+----------------------------------+--------+---------+----------+----------+--------------+
| 1  | High-level design doc            | 1d     |         |          |          |              |
+----+----------------------------------+--------+---------+----------+----------+--------------+
| 2  | Security review                  | 1d     |         |          |          |              |
+----+----------------------------------+--------+---------+----------+----------+--------------+
| 3  | Update architecture doc          | 4d     |         |          |          |              |
+----+----------------------------------+--------+---------+----------+----------+--------------+
| 4  | Add cache key to interface model | 1d     | lebaron | DVR      | 1, 2     |              |
+----+----------------------------------+--------+---------+----------+----------+--------------+
```

Now that we have a few things configured, let's take a look at a gantt chart.

## Gantt

`rantt` is all about Gantt! In fact, producing a gantt chart is the default behavior if no other sub-commands are specificed. Let's try it out.

```
% rantt

+----+--------------------------------------------+--------+--------------+-----------------+-----------------+-----------------+-----------------+
| DVR
+----+--------------------------------------------+--------+--------------+-----------------+-----------------+-----------------+-----------------+
| ID | Description                                | Effort | Dependencies | Aug 02 - Aug 07 | Aug 09 - Aug 14 | Aug 16 - Aug 21 | Aug 23 - Aug 28 |
+----+--------------------------------------------+--------+--------------+-----------------+-----------------+-----------------+-----------------+
| 4  | Add cache key to interface model [lebaron] | 1d     | 1, 2         |    ===          |                 |                 |                 |
+----+--------------------------------------------+--------+--------------+-----------------+-----------------+-----------------+-----------------+
| No Project
+----+--------------------------------------------+--------+--------------+-----------------+-----------------+-----------------+-----------------+
| ID | Description                                | Effort | Dependencies | Aug 02 - Aug 07 | Aug 09 - Aug 14 | Aug 16 - Aug 21 | Aug 23 - Aug 28 |
+----+--------------------------------------------+--------+--------------+-----------------+-----------------+-----------------+-----------------+
| 1  | High-level design doc                      | 1d     |              | ===             |                 |                 |                 |
+----+--------------------------------------------+--------+--------------+-----------------+-----------------+-----------------+-----------------+
| 2  | Security review                            | 1d     |              | ===             |                 |                 |                 |
+----+--------------------------------------------+--------+--------------+-----------------+-----------------+-----------------+-----------------+
| 3  | Update Arch doc                            | 4d     |              | ============    |                 |                 |                 |
+----+--------------------------------------------+--------+--------------+-----------------+-----------------+-----------------+-----------------+
| 4  | Add cache key to interface model [lebaron] | 1d     | 1, 2         |    ===          |                 |                 |                 |
+----+--------------------------------------------+--------+--------------+-----------------+-----------------+-----------------+-----------------+
```
